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
Cへ関数シンボルをexportするために`#[no_mangle]`を使いたくなるかもしれないが、
可変長引数関数の関数ポインタを期待するCコードにその関数を渡すこともある
???

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

LLVM already provides a set of intrinsics, implementing `va_start`, `va_arg`,
`va_end`, and `va_copy`. The compiler will insert a call to the `va_start`
intrinsic at the start of the function to provide the `VaList` argument (if
used), and a matching call to the `va_end` intrinsic on any exit from the
function. The implementation of `VaList::arg` will call `va_arg`. The
implementation of `VaList::copy` wil call `va_copy`, and then `va_end` after
the closure exits.

`VaList` may become a language item (`#[lang="VaList"]`) to attach the
appropriate compiler handling.

The compiler may need to handle the type `VaList` specially, in order to
provide the desired parameter-passing semantics at FFI boundaries. In
particular, some platforms define `va_list` as a single-element array, such
that declaring a `va_list` allocates storage, but passing a `va_list` as a
function parameter occurs by pointer. The compiler must arrange to handle both
receiving and passing `VaList` parameters in a manner compatible with the C
ABI.

The C standard requires that the call to `va_end` for a `va_list` occur in the
same function as the matching `va_start` or `va_copy` for that `va_list`. Some
C implementations do not enforce this requirement, allowing for functions that
call `va_end` on a passed-in `va_list` that they did not create. This RFC does
not define a means of implementing or calling non-standard functions like these.

Note that on some platforms, these LLVM intrinsics do not fully implement the
necessary functionality, expecting the invoker of the intrinsic to provide
additional LLVM IR code. On such platforms, rustc will need to provide the
appropriate additional code, just as clang does.

This RFC intentionally does not specify or expose the mechanism used to limit
the use of `VaList::arg` only to specific types. The compiler should provide
errors similar to those associated with passing types through FFI function
calls.

# Drawbacks
[drawbacks]: #drawbacks

This feature is highly unsafe, and requires carefully written code to extract
the appropriate argument types provided by the caller, based on whatever
arbitrary runtime information determines those types. However, in this regard,
this feature provides no more unsafety than the equivalent C code, and in fact
provides several additional safety mechanisms, such as automatic handling of
type promotions, lifetimes, copies, and cleanup.

# Rationale and Alternatives
[alternatives]: #alternatives

This represents one of the few C-compatible interfaces that Rust does not
provide. Currently, Rust code wishing to interoperate with C has no alternative
to this mechanism, other than hand-written C stubs. This also limits the
ability to incrementally translate C to Rust, or to bind to C interfaces that
expect variadic callbacks.

Rather than having the compiler invent an appropriate lifetime parameter, we
could simply require the unsafe code implementing a variadic function to avoid
ever allowing the `VaList` structure to outlive it. However, if we can provide
an appropriate compile-time lifetime check, doing would make it easier to
correctly write the appropriate unsafe code.

Rather than naming the argument in the variadic function signature, we could
provide a `VaList::start` function to return one. This would also allow calling
`start` more than once. However, this would complicate the lifetime handling
required to ensure that the `VaList` does not outlive the call to the variadic
function.

We could use several alternative syntaxes to declare the argument in the
signature, including `...args`, or listing the `VaList` or `VaList<'a>` type
explicitly. The latter, however, would require care to ensure that code could
not reference or alias the lifetime.

# Unresolved questions
[unresolved]: #unresolved-questions

When implementing this feature, we will need to determine whether the compiler
can provide an appropriate lifetime that prevents a `VaList` from outliving its
corresponding variadic function.

Currently, Rust does not allow passing a closure to C code expecting a pointer
to an `extern "C"` function. If this becomes possible in the future, then
variadic closures would become useful, and we should add them at that time.

This RFC only supports the platform's native `"C"` ABI, not any other ABI. Code
may wish to define variadic functions for another ABI, and potentially more
than one such ABI in the same program. However, such support should not
complicate the common case. LLVM has extremely limited support for this, for
only a specific pair of platforms (supporting the Windows ABI on platforms that
use the System V ABI), with no generalized support in the underlying
intrinsics. The LLVM intrinsics only support using the ABI of the containing
function. Given the current state of the ecosystem, this RFC only proposes
supporting the native `"C"` ABI for now. Doing so will not prevent the
introduction of support for non-native ABIs in the future.
