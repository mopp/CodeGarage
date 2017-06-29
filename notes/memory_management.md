# 自作OSからみたメモリ管理概論
なお、実装出来てない

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

# フラグメントの話
* 内部フラグメント/外部フラグメント

# Reference
* http://wiki.osdev.org/Memory_management
* http://wiki.osdev.org/Brendan%27s_Memory_Management_Guide
* http://wiki.osdev.org/User:Pancakes/SimpleHeapImplementation
* http://wiki.osdev.org/Page_Frame_Allocation#Hybrid_scheme


# 考えたこと
メモリ使用者側がどのくらいのライフタイムで使用するかによって
どういうメモリアロケートの戦略にするかを変えられる？
例えば、ライフタイムが小さいのであれば、直ぐにメモリは返却されるため
フラグメントに対する考慮はいらないはず
逆にライフタイムが非常に長いのであれば、多少のオーバーヘッドが付随してもフラグメントを回避することで
長期で見たとき、効率的なメモリ管理を実現できるのではないか？
これを実現するためには？
malloc(request_size, alignment, lifetime)
というふうに確保をしてやる
そもそも、mallocに与える情報が少なすぎるのではないか
メモリマネージャマネージャが必要になるか？
多段になることで実行コストが増加するか？
論文ありそう
