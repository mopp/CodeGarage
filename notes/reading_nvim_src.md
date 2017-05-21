# Neovimのソースを読む
* wikiの`Developers`の項目にC言語のtipsやneovim固有ルールなどが書いてあるので参照しながら読む
* `src/nvim/README.md`も参照
* Vimは入力駆動ステートマシン(より具体的にはプッシュダウン・オートマトン)
* insert/normal/visualなど以外に内部実装の都合で入力待ち状態などを持つ
    * `vim_state`構造体がそのためのコールバック関数と状態を持つ

# 便利リンク
* [Neovim - Wiki](https://github.com/neovim/neovim/wiki)
* [Neovim Style Guide](https://neovim.io/develop/style-guide.xml)
* [C programming techniques](https://github.com/neovim/neovim/wiki/C-programming)
* [Refactor vim into a library](https://github.com/neovim/neovim/wiki/Refactor-vim-into-a-library)
    * Neovimをキーを処理するだけのステートマシンとして見る
* [Code overview](https://github.com/neovim/neovim/wiki/Code-overview)
* [Development-tips](https://github.com/neovim/neovim/wiki/Development-tips)


# Code Overview
Code Overview(リンク一覧を参照)にソース全体の説明が書いてある

## 重要な変数
* `State`: 現在のneovimの状態を持つ
* `curwin`: 現在のウィンドウ
* `curbuf`: 現在のバッファ

その他のグローバル変数は、[globals.h](https://github.com/neovim/neovim/blob/master/src/nvim/globals.h)に書いてある
例外もあるっぽい？

## The main loop
ソースコード中では`main_loop`変数のこと
`struct Loop`型

この関数が、更新処理などをして、`normal_cmd`関数を呼び出す
`normal_cmd`関数はコマンドが終了したら戻ってくる
基本的な考えとして、neovimは1文字入力されるまでユーザを待つ、そして、新たな文字が必要となるまで処理を行う
ゆえに、ソースのいくつかの部分で文字入力待ちが存在して、`vgetc`関数がそのためによく使用される
また、`vgetc`関数はマッピングを処理するためにも使用される

殆どの場合、画面更新はコマンド全てが終了するまで遅延させられる
これは`win_update`関数や`win_line`関数を呼び出す`update_screen`関数が行う
詳細は`screen.c`が行う


# 読んでいる間のメモ

## main.c
* `nvim_main`と`main`が`MAKE_LIB`で`ifdef`されているので`tagbar`するときに注意
* このファイル内でのみ`INIT`マクロが展開され、`EXTERN`が空に展開される
* それ以外の場所では`INIT`マクロが消され、`EXTERN`が`extern`になる
* `main`関数内でまずは初期化処理
    * 時間モジュールの初期化
    * `params`変数の初期化
        * `mparm_T`構造体の中によく使用される引数を詰め込む
            * `argc`, `argv`もこの中にある
            * これによって、関数の引数部を短く書ける
    * `--startuptime`のための初期化
    * `early_init` - unit testsが必要らしい
        * ここの中でもいろいろ初期化がある
        * table of normal mode command ?
        * 1つめのウィンドウとバッファーの確保
        * `init_yank`
        * `set_init_1`: Initialize the options, first part.
    * `event_init`
    * `command_line_scan`
        * コマンドライン引数の処理
    * `win_init_size`
    * `screenalloc`
        * シェル上に表示されているもの全体を指してスクリーンと言っている？
    * `set_init_2`
    * `init_highlight`
* `"-u NONE`があれば`loadplugins`をfalseにする
* `source_startup_scripts`関数でユーザスクリプトを読み込み
    * `do_user_initialization`: Source vimrc or do other user initialization
    * `do_source`: Read the file "fname" and execute its lines as EX commands.

