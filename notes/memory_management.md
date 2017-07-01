# 自作OSからみたメモリ管理概論
なお、実装出来てない

# 全ては実装されていないことに注意

# OSのメモリ管理は3つに分けられる
* ヒープメモリ管理
* 仮想メモリ管理
* 物理メモリ管理

# ヒープメモリ管理
* 一般に動的メモリ管理と言ったらこちらを指す
* いわゆるmalloc/free
* ただしkernel内でも同じようなことはしたいのでk`kmalloc`と`kfree`がある (Linuxの場合)

# 仮想メモリ管理
* いろんなふりをする (virtualとか仮想的ではなく、事実上の、という意味！)
    * Copy on Write / on demand
    * File (IOを減らす、書き込みのときとか有効)
    * 物理メモリよりも多くのメモリがあるフリ
* 物理メモリを共有させて、プロセス間のやりとりを行う
* セキュリティの維持
    * データの排他性
* 仮想メモリ管理をするために必要なこと
* 初心者がよくやる過ち
    * 物理メモリをカーネル空間にそのままマップする

# 物理メモリ管理
    * 仮想メモリ管理と同じ感じ
    * memory mapped PCIデバイスなど、メモリでないものも管理
    * 物理メモリの仕様のため、特殊な要求を受け付けなければならない場合もある
        * 連続でないといけない
        * 特定アドレスの近傍でないといけない
            * legacy/ISA DMA controllers, where the buffer has to be below 0x01000000, must be physically contiguous and must not cross a 64 KiB boundary.
        * これらを考慮すると、アロケータのパフォーマンスは落ちる
        * なので、メモリ空間をZONEに分割することがおすすめ
    * 物理メモリ管理をするために必要なこと

# フラグメントの話
* 内部フラグメント/外部フラグメント

# Reference
* http://wiki.osdev.org/Memory_management
* http://wiki.osdev.org/Brendan%27s_Memory_Management_Guide
* http://wiki.osdev.org/User:Pancakes/SimpleHeapImplementation
* http://wiki.osdev.org/Page_Frame_Allocation#Hybrid_scheme


# 考えたこと
* メモリ確保時にlifetime情報も渡せばいいのでは？
    * メモリ使用者側がどのくらいのライフタイムで使用するかによって
    * どういうメモリアロケートの戦略にするかを変えられる？
    * 例えば、ライフタイムが小さいのであれば、直ぐにメモリは返却されるため
    * フラグメントに対する考慮はいらないはず
    * 逆にライフタイムが非常に長いのであれば、多少のオーバーヘッドが付随してもフラグメントを回避することで
    * 長期で見たとき、効率的なメモリ管理を実現できるのではないか？
    * これを実現するためには？
    * malloc(request_size, alignment, lifetime)
    * というふうに確保をしてやる
    * そもそも、mallocに与える情報が少なすぎるのではないか
    * メモリマネージャマネージャが必要になるか？
    * 多段になることで実行コストが増加するか？
    * 論文ありそう
* PageFaultが起きたとき、プロセスをwaitさせられる
    * Linuxがやってたっけ？
    * メモリ不足の場合など、とりあえず仮想メモリを返しておいて
    * PageFault内で実際にメモリが空くまで待たせる
* メモリアロケータアンサンブル
