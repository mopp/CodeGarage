[rfcs/2137-variadic.md at master · rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/master/text/2137-variadic.md)

- Feature Name: variadic
- Start Date: 2017-08-21
- RFC PR: https://github.com/rust-lang/rfcs/pull/2137
- Rust Issue: https://github.com/rust-lang/rust/issues/44930


# 要約
C互換の可変長引数関数をRustでサポートする, via new intrinsics ([std::intrinsics - Rust](https://doc.rust-lang.org/std/intrinsics/))
現時点ではexternalな可変長引数関数の宣言と呼び出し(unsafe)をサポートしている
しかし、Rustで可変長引数関数を直接書くことは出来ない
新たにRustがこれをサポートすることで、以下の利点がある

    * Rustによるより多くのCライブラリの置き換え
    * Cコードを要求することとプラットフォーム固有コードの再実装でのerror-proneの回避
    * Cコードベースのインクリメンタルトランスレーションの改善
    * 可変長引数コールバックが実装可能


# 動機
Rustは全てのCインターフェースの呼び出しと、Cから呼ばれる**殆どの**インターフェースをexportできる
可変長引数関数は出来ないものの一つ
Cから呼び出せる可変長引数関数を書くには、C側で関数を書き、Rustプログラムにリンクし、その関数がRustを呼び出すようにしないといけない
更に、引数が`va_list`構造体に入っていたとしても引数を抜き出す処理が例外的にerror-prone、プラットフォーム固有コードが必要になる
いくつかの対象アーキテクチャのための部分的解決方法はcrates.ioで提供されている

このRFCでは、ネイティブRustコードから可変長の引数をネイティブRust関数に渡すためのインターフェースは提案しない
また、あらゆる種類の型安全性を提供するインターフェースも提案しない.
この提案は主に、RustがCから呼び出し可能なインターフェースを提供可能にするためにある


# ガイドレベルの説明
C言語の可変長引数に習って、Rustも以下のように最後の引数に`...`を付けて可変長引数を表す

```rust
pub unsafe extern "C" fn func(arg: T, arg2: T2, mut args: ...) {
    // implementation
}
```

`...`は最後の引数でないといけない
その前に少なくとも1つは引数を取らないといけない
`extern "C"`と`unsafe`も必須
Cコードが直接呼び出すための関数をexportするために`#[no_mangle]`を付けることで
Rustから可変長引数関数のポインタを要求するCコードにそのexportした関数を渡せるようになる

`args`の型は`core::intrinsics::VaList<'a>`になる
コンパイラはライフタイム`'a`を引数が可変長関数よりも長く生存することを防ぐために提供する

引数にアクセスするため、以下のインターフェースを`core::intrinsics`で提供する

```rust
/// The argument list of a C-compatible variadic function, corresponding to the
/// underlying C `va_list`. Opaque.
pub struct VaList<'a> { /* fields omitted */ }

// Note: the lifetime on VaList is invariant
impl<'a> VaList<'a> {
    /// Extract the next argument from the argument list. T must have a type
    /// usable in an FFI interface.
    pub unsafe fn arg<T>(&mut self) -> T;

    /// Copy the argument list. Destroys the copy after the closure returns.
    pub fn copy<'ret, F, T>(&self, F) -> T
    where
        F: for<'copy> FnOnce(VaList<'copy>) -> T, T: 'ret;
}
```

`VaList::arg`の戻り値の型は`extern "C"`FFIインターフェース内で有効な型でなければならない
the compiler allows all the same types returned from `VaList::arg` that it allows in the function signature of an `extern "C"` function.

`libc`クレート内のCの整数型と浮動小数型に相当するもの全てはRustの型へのエイリアスなので、`VaList::arg`で取り出せる

引数の取り出しはCの引数渡しと拡張のルールに従う
特に、Cは`int`より小さい型は`int`に拡張する、`float`は`double`に拡張する
なので、Rustでのそれらの型の引数取り出しは`int`か`double`で取り出す、もしくは適切に変換される

Cの`va_list`と同じようにプラットフォーム固有表現になるので、`VaList`は不透明型になる

可変長引数関数から`VaList`を他の関数に渡すことは出来るが
ライフタイムがあるので`VaList`のreturnや、関数自体より長く生存することは出来ない
`copy`から呼ばれるクロージャーも同様

Cの`printf`を`VaList`を使ってRustで宣言する例
`VaList`がCの`va_list`に対応する

```rust
extern "C" {
    pub unsafe fn vprintf(format: *const c_char, ap: VaList) -> c_int;
    pub unsafe fn vfprintf(stream: *mut FILE, format: *const c_char, ap: VaList) -> c_int;
    pub unsafe fn vsprintf(s: *mut c_char, format: *const c_char, ap: VaList) -> c_int;
    pub unsafe fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, ap: VaList) -> c_int;
}
```

渡した`VaList`はその後使用できない
引数として渡した後にも使用したい場合は`VaList::copy`を使用する

Rustの`unsafe extern "C"`な関数は`VaList`を引数として取れる
そのような関数はライフタイムを指定してはいけない

以上の機能を使用するには`c_variadic`featureが必要

例:

```rust
#![feature(c_variadic)]

#[no_mangle]
pub unsafe extern "C" fn func(fixed: u32, mut args: ...) {
    let x: u8 = args.arg();
    let y: u16 = args.arg();
    let z: u32 = args.arg();
    println!("{} {} {} {}", fixed, x, y, z);
}
```

Cからの呼び出し例:

```c
#include <stdint.h>

void func(uint32_t fixed, ...);

int main(void)
{
    uint8_t x = 10;
    uint16_t y = 15;
    uint32_t z = 20;
    func(5, x, y, z);
    return 0;
}
```

実行例

```text
5 10 15 20
```

# リファレンスレベルの説明
[reference-level-explanation]: #reference-level-explanation

LLVMが既に`va_start`, `va_arg`, `va_end`, `va_copy`を提供している
`VaList`が関数内で使用されれば、コンパイラが`va_start`への呼び出しを先頭に追加し、関数が終了するときに`va_end`も呼ぶようにする
`VaList::arg`の実装は`va_arg`への呼び出しになる
`VaList::copy`の実装は`va_copy`になり、クロージャ終了時に、コピーした引数の分の`va_end`を呼ぶ

`VaList`は恐らくコンパイラが適切に処理するためにlanguage item (`#[lang="VaList"]`)となる

FFI境界で望まれるparameter-passing semanticsを提供するため、コンパイラは`VaList`型を特別に処理することになる
特に、一部のプラットフォームでは`va_list`を単一要素配列として定義している
つまり、`va_list`を宣言するとstorageが割当られるが、`va_list`はポインタとして渡される
! 構造体の中に動的に確保した配列のポインタが入ってるだけで、それを引数に渡している、という意味かと思う
コンパイラはC ABI互換を保ちつつ引数受け取りと引数渡しをしなければならない

Cの規格では`va_end`は`va_start`か`va_copy`を呼び出した関数内で実行される必要がある
しかし、幾つかのCの実装はこれを強制しない
このRFCではそれらの実装、呼び出しについては定義しない

幾つかのプラットフォームで、LLVM組み込み関数の実装が完全ではないのため、呼び出し側が追加のLLVM IRを提供することを期待している
それらのプラットフォームでは`rustc`も`clang`がしているように適切な追加LLVM IRを提供する必要がある

このRFCは`VaList::arg`の使用を特定の型に制限するようなメカニズムを意図的に書かないようにしている
コンパイラはFFI関数呼び出しを通して渡される型に関連づいたエラーを提供しなければならない


# 欠点

このfeatureは強くunsafeで、呼び出し側が指定した適切な引数型を取り出すコードは、任意のランタイム情報にもとづいて、注意深く書かれる必要がある
しかし、この点においては、このfeatureは同等のCコードと同じくらいにunsafeであるが、安全性のためのメカニズムを幾つか追加する
例えば、型拡張の自動適用、ライフタイム、コピー、クリーンアップなど
! コピーとクリーンアップは`va_copy`と`va_end`の呼び出しをコンパイラが挿入することだと思われる


# 論拠と代替案

このRFCはRustが提供していないC互換インターフェースの提案である
現時点では、Cと相互運用されることを望まれるRustコードは、Cを書く以外の代替案を持たない
これはインクリメンタルにCをRustに書き直すこと、もしくは、可変長引数コールバックを期待するCインターフェースの使用を制限している

コンパイラが適切なライフタイムをでっち上げるよりも、我々が`VaList`がoutliveすることを避ける可変長引数関数をunsafeで実装する方法もある
しかし、もし、我々が適切なコンパイル時ライフタイムチェックを提供できるなら、適切なunsafeコードを書くことをより簡単にする
! ?

可変長引数関数の引数に名前を付けるよりも、引数を返す`VaList::start`関数を定義するほうがいいかもしれない
これによって、`start`を複数回呼び出せるようになるが、ライフタイムの制御が難しくなる

`...args`や、型を明示する`VaList`、`VaList<'a>`などの、引数宣言の別な構文を使ってもいい
後者ではライフタイムの参照やエイリアスがないことを保証する必要がある
! ユーザがライフタイムを渡したり、意味が無いので出来ないようにしたい


# 未解決の疑問

このfeatureを実装するとき、コンパイラが適切なライフタイムを`VaList`に付加できるかを判断する必要がある

現時点で、Rustは`extern "C"`関数へのポインタを期待するCコードにクロージャを渡すことはできない
将来可能になったとき、可変長引数クロージャは便利になるはずなので、そのとき追加する

このRFCはプラットフォームのネイティブなC ABIのみをサポートし、他のABIはサポートしない
同じプログラム内で他のABIへの可変長引数関数を定義したくなるかもしれない
しかし、それらのサポートで一般的なケースを複雑にすべきではない
LLVMでは極めて限定的にこれをサポートしているのみで、一般的なサポートはない (System V ABIを使用するプラットフォーム上でのWindows ABIサポートなどのみ)
LLVM組み込み関数は、包含する関数のABIのみをサポートしている
エコシステムの現状を考えて、このRFCでは今のところネイティブC ABIをサポートすることのみを提案している
そうすることで将来的に非ネイティブABIのサポートの導入を妨げないようにする
