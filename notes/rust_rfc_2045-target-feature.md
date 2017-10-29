[rfcs/2045-target-feature.md at master · rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/master/text/2045-target-feature.md)

- Feature Name: target_feature / cfg_target_feature / cfg_feature_enabled
- Start Date: 2017-06-26
- RFC PR: rust-lang/rfcs#2045
- Rust Issue: rust-lang/rust#44839


# 動機と要約
`x86_64`とか`ARMv8`とか、ざっくりとしたアーキテクチャの指定がある
標準だと、Rustコンパイラは指定されたアーキテクチャに含まれる全CPUで動作するようなバイナリを生成する
コンパイラフラグ`--target-feature`と`--target-cpu`を使って、どのCPUでバイナリが実行されるか知っているユーザは拡張命令を使える
指定したものと異なるCPUで、そのバイナリを実行した場合の動作は未定義動作である
現在、実行CPUがわかっても、stable Rustでは以下のことが出来ない
! `target_arch`, `target_os`, `target_family`などは既にある ([Attributes - The Rust Reference](https://doc.rust-lang.org/reference/attributes.html))

* コンパイル時にどのfeaturesが有効か決定すること
* 実行時にどのfeaturesが有効か決定すること
* 同じバイナリへ、異なるfeaturesのセットのコードを埋め込むこと

そのようなプログラムは、有効なfeaturesによって、異なるアルゴリズムを使用でき、特定のアーキテクチャ上の多くのCPUファミリで効率的に実行できるポータブルなrustバイナリを生成可能になる

このRFCの目的は、Rustを上記の3つ問題を解決するために拡張することであり、それらは以下の3つのlanguage featuresを追加することで実現する

* compile-time feature detection
    * configurationマクロ、`cfg!(target_feature = "avx2")`を使って、featureが有効か無効かを検出する
* run-time feature detection
    * API `cfg_feature_enabled!("avx2")` を使って、現在のホストがfeatureをサポートしているか検出する
* unconditional code generation
    * 関数アトリビュート `#[target_feature(enable = "avx2")]` を使って、ホストがfeatureをサポートしていたときのみ到達するという仮定の元、コンパイラにコード生成させる


# 詳細デザイン

## Target feature

rustcの各targetは持っているはバックエンドのコンパイルオプションで制御可能なtarget featuresの標準セットを持つ
それらtarget featuresはコンパイラかLLVMなどのバックエンドでドキュメント化されるべき

このRFCは如何なるtarget featuresも追加しない代わりに、target featuresを追加するプロセスを指定する
それぞれのtarget featureは: 

- 自身のmini-RFC, RFC, rustcのissueで提案されなければならず、a FCP periodに従わなくてはならない
- feature gate macroは`target_feature_feature_name`の形式にしなくてはならない (`feature_name`を名前で置き換える)
- できるならば、`cfg_feature_enabled!("name")` APIで実行時に検出可能にしなければならない
- バックエンド固有のコンパイルオプションがこの機能を有効にする必要があるかどうかを含めなければならない

nightlyで unstable target features を使うために、crateは `#![allow(target_feature_avx2)]` のように記述しなければならない
1フルリリースサイクルの猶予期間が与えられ、hard errorになる前に、soft errorを起こすようになる

## Backend compilation options

現在のstableでtarget featureをコード生成バックエンドに渡す方法は2つある: 

- `-C --target-feature=+/-backend_target_feature_name`
    + ここで `+/-` は標準featureセットへの追加と削除を意味する

- `-C --target-cpu=backend_cpu_name`
    + crateの標準featureセットを`backend_cpu_name`で有効になっている全てのfeatureのものへと変更する

! ` rustc -C target-cpu=core2` ハイフン2つがいらない？

これらのセマンティクスはLLVM固有で、LLVMがどう扱うかに依存している

それらオプションはそのままにし、新しいコンパイラオプション`--enable-features="feature0,feature1,..."`を追加する
    + stableなtarget featureのみサポート
    + `--disable-features`は検討中
    + 後方互換をこれで保つ
    + オプションでfeatureが指定されたとき、`cfg!(target_feature = "feature")`と`cfg_feature_enabled!("feature")`が`true`になる

バックエンドコンパイルオプション`-C --target-feature/--target-cpu`stableなfeaturesを有効にするかどうかは、別のRFCで解決されるべき

## Unconditional code generation: `#[target_feature]`

(note: 関数アトリビュートの`#[target_feature]`はclangやgccの[`__attribute__ ((__target__ ("feature")))`](https://clang.llvm.org/docs/AttributeReference.html#target-gnu-target)に似ている)

このRFCではunsafeな関数にのみ適用可能なアトリビュートを導入する
[`#[target_feature(enable = "feature_list")]`](https://github.com/rust-lang/rust/pull/38079) 
    ! `#[target_feature = "+sse4.2"]`を付けた関数内でのみSSE 4.2の命令が使えるようになる
    ! `-C target-feature or -C target-cpu` は不要、ただし呼び出し制限など一切無し
    ! merge済

- このアトリビュートは関数のfeature setを拡張し、ハードウェアがサポートしている場合のみに関数が呼ばれるという想定の元でコンパイラにコード生成させる
- サポートサれていないターゲット上での呼び出しは未定義動作
- すべての関数のfeaturesをサポートしていないコンテキスト上では、コンパイラは関数をinline化しない
    * あるfeatureを使う関数が、そのfeatureが使えるかどうかわからない場所ではinline化されない、という意味のはず
- `#[target_feature(enable = "feature")]` が付いた関数では`cfg!(target_feature = "feature")`と`cfg_feature_enabled!("feature")`は`true`になる

Note 0: 現在のRFCでは如何なるstable RustのABI issuesを導入しない (詳細は未解決の問題へ)

Note 1: 関数が`#[target_feature]`付きで定義されている
関数が指定されたfeature setを拡張するコンテキストへinline化されるとき、コンパイラはその関数を拡張されたfeature setを使ってコード生成してもよい (逆のときはinline化禁止)

**Example 0 (basics):**

* `#[target_feature]`の使い方の例
* CPUのサポートするfeatureに応じて、異なる関数実装の実行時検出と呼び出しの例

```rust
// This function will be optimized for different targets
#[inline(always)] fn foo_impl() { ... }

// This generates a stub for CPUs that support SSE4:
#[target_feature(enable = "sse4")] unsafe fn foo_sse4() {
    // Inlining `foo_impl` here is fine because `foo_sse4`
    // extends `foo_impl` feature set
    foo_impl()
}

// This generates a stub for CPUs that support AVX:
#[target_feature(enable = "avx")] unsafe fn foo_avx() { foo_impl() }

// This function returns the best implementation of `foo` depending
// on which target features the host CPU does support at run-time:
fn initialize_global_foo_ptr() -> fn () -> () {
    if cfg_feature_enabled!("avx") {
      unsafe { foo_avx }
    } else if cfg_feature_enabled!("sse4") {
      unsafe { foo_sse4 }
    } else {
      foo_impl // use the default version
    }
}

// During binary initialization we can set a global function pointer
// to the best implementation of foo depending on the features that
// the CPU where the binary is running does support:
lazy_static! {
    static ref GLOBAL_FOO_PTR: fn() -> () = {
        initialize_foo()
    };
}
// ^^ note: the ABI of this function pointer is independent of the target features


fn main() {
  // Finally, we can use the function pointer to dispatch to the best implementation:
  global_foo_ptr();
}
```

! main関数での呼び出しは`GLOBAL_FOO_PTR()`では？

**Example 1 (inlining):**

```rust
#[target_feature(enable = "avx")] unsafe fn foo();
#[target_feature(enable = "avx")] #[inline] unsafe fn baz(); // OK
#[target_feature(enable = "avx")] #[inline(always)] unsafe fn bar(); // OK

#[target_feature(enable = "sse3")]
unsafe fn moo() {
  // This function supports SSE3 but not AVX
  if cfg_feature_enabled!("avx") {
      foo(); // OK: foo is not inlined into moo
      baz(); // OK: baz is not inlined into moo
      bar();
      // ^ ERROR: bar cannot be inlined across mismatching features
      // did you meant to make bar #[inline] instead of #[inline(always)]?
      // Note: the logic to detect this is the same as for the call
      // to baz, but in this case rustc must emit an error because an
      // #[inline(always)] function cannot be inlined in this call site.
  }
}
```

## Conditional compilation: `cfg!(target_feature)`

現在のコンテキストであるfeatureが有効かどうかを[`cfg!(target_feature = "feature_name")`](https://github.com/rust-lang/rust/issues/29717) マクロによって参照できる
有効であれば`true`、無効であれば`false`を返す

もし生成されるコードがfeatureをサポートしていれば、`#[target_feature(enable = "feature_name")]`が付いた関数内で、`cfg!(target_feature = "feature_name")`マクロは `true`に展開される
[current bug](https://github.com/rust-lang/rust/issues/42515).
code for the function uses the feature 

Note: 

Note: how accurate `cfg!(target_feature)` can be made is an "Unresolved Question" (see the section below).
Ideally, when `cfg!(target_feature)` is used in a function that does not support the feature, it should still return true in the cases where the function gets inlined into a context that does support the feature.
This can happen often if the function is generic, or an `#[inline]` function defined in a different crate.
This can results in errors at monomorphization time only if `#![cfg(target_feature)]` is used, but not if `if cfg!(target_feature)` is used since in this case all branches need to type-check properly.

**Example 3 (conditional compilation):**

```rust
fn bzhi_u32(x: u32, bit_position: u32) -> u32 {
    // Conditional compilation: both branches must be syntactically valid,
    // but it suffices that the true branch type-checks:
    #[cfg(target_feature = "bmi2")] {
        // if this code is being compiled with BMI2 support, use a BMI2 instruction:
        unsafe { intrinsic::bmi2::bzhi(x, bit_position) }
    }
    #[cfg(not(target_feature = "bmi2"))] {
        // otherwise, call a portable emulation of the BMI2 instruction
        portable_emulation::bzhi(x, bit_position)
    }
}

fn bzhi_u64(x: u64, bit_position: u64) -> u64 {
    // Here both branches must type-check and whether the false branch is removed
    // or not is left up to the optimizer.
    if cfg!(target_feature = "bmi2") {  // `cfg!` expands to `true` or `false` at compile-time
        // if target has the BMI2 instruction set, use a BMI2 instruction:
        unsafe { intrinsic::bmi2::bzhi(x, bit_position) }
        // ^^^ NOTE: this function cannot be inlined unless `bzhi_u64` supports
        // the required features
    } else {
        // otherwise call an algorithm that emulates the instruction:
        portable_emulation::bzhi(x, bit_position)
    }
}
```

**Example 4 (value of `cfg!` within `#[target_feature]`):**

```rust
#[target_feature("+avx")]
unsafe fn foo() {
  if cfg!(target_feature = "avx") { /* this branch is always taken */ }
  else { /* this branch is never taken */ }
  #[cfg(not(target_feature = "avx"))] {
    // this is dead code
  }
}
```

## Run-time feature detection

Writing safe wrappers around `unsafe` functions annotated with
`#[target_feature]` requires run-time feature detection. This RFC adds the following
macro to the standard library:

- `cfg_feature_enabled!("feature") -> bool-expr`

with the following semantics: "if the host hardware on which the current code is running
supports the `"feature"`, the `bool-expr` that `cfg_feature_enabled!` expands to has
value `true`, and `false` otherwise.

If the result is known at compile-time, the macro approach allows expanding the result
without performing any run-time detection at all. This RFC does not guarantee that this
is the case, but [the current implementation](https://github.com/rust-lang-nursery/stdsimd)
does this.

Examples of using run-time feature detection have been shown throughout this RFC, there
isn't really more to it.

If the API of run-time feature detection turns out to be controversial before
stabilization, a follow-up RFC that focus on run-time feature detection will need
to be merged, blocking the stabilization of this RFC.

# How We Teach This
[how-we-teach-this]: #how-we-teach-this

There are two parts to this story, the low-level part, and the high-level part.

**Example 5 (high-level usage of target features):**

**note**: `ifunc` is not part of this RFC, but just an example of what can be built on top of it.

In the high-level part we have the `ifunc` function attribute, implemented as a procedural macro (some of these macros [already](https://github.com/alexcrichton/cfg-specialize/blob/master/cfg-specialize-macros) [exist](https://github.com/parched/runtime-target-feature-rs)):

```rust
#[ifunc("default", "sse4", "avx", "avx2")]  //< MAGIC
fn foo() {}

fn main() {
  foo(); // dispatches to the best implementation at run-time
  #[cfg(target_feature = "sse4")] {
    foo(); // dispatches to the sse4 implementation at compile-time
  }
}
```

The following example covers what `ifunc` might expand to.

**Example 6 (ifunc expansion):**

```rust
// Copy-pastes "foo" and generates code for multiple target features:
unsafe fn foo_default() { ...foo tokens... }
#[target_feature(enable = "sse4")] unsafe fn foo_sse4() { ...foo tokens... }
#[target_feature(enable = "avx")]  unsafe fn foo_avx() { ...foo tokens... }
#[target_feature(enable = "avx2")] unsafe fn foo_avx2() { ...foo tokens... }

// Initializes `foo` on binary initialization
static foo_ptr: fn() -> () = initialize_foo();

fn initialize_foo() -> typeof(foo) {
    // run-time feature detection:
    if cfg_feature_enabled!("avx2")  { return unsafe { foo_avx2 } }
    if cfg_feature_enabled!("avx")  { return unsafe { foo_avx } }
    if cfg_feature_enabled!("sse4")  { return unsafe { foo_sse4 } }
    foo_default
}

// Wrap foo to do compile-time dispatch
#[inline(always)] fn foo() {
  #[cfg(target_feature = "avx2")]
  { unsafe { foo_avx2() } }
  #[cfg(and(target_feature = "avx"), not(target_feature = "avx2")))]
  { unsafe { foo_avx() } }
  #[cfg(and(not(target_feature = "sse4")), not(target_feature = "avx")))]
  { unsafe { foo_sse4() } }
  #[cfg(not(target_feature = "sse4"))]
  { foo_ptr() }
}
```

Note that there are many solutions to this problem and they have different
trade-offs, but these can be explored in procedural macros. When wrapping unsafe
intrinsics, conditional compilation can be used to create zero-cost wrappers:

**Example 7 (three-layered approach to target features):**

```rust
// Raw unsafe intrinsic: in LLVM, std::intrinsic, etc.
// Calling this on an unsupported target is undefined behavior.
extern "C" { fn raw_intrinsic_function(f64, f64) -> f64; }

// Software emulation of the intrinsic,
// works on all architectures.
fn software_emulation_of_raw_intrinsic_function(f64, f64) -> f64;

// Safe zero-cost wrapper over the intrinsic
// (i.e. can be inlined)
fn my_intrinsic(a: f64, b: f64) -> f64 {
  #[cfg(target_feature = "some_feature")] {
    // If "some_feature" is enabled, it is safe to call the
    // raw intrinsic function
    unsafe { raw_intrinsic_function(a, b) }
  }
  #[cfg(not(target_feature = "some_feature"))] {
     // if "some_feature" is disabled calling
     // the raw intrinsic function is undefined behavior (per LLVM),
     // we call the safe software emulation of the intrinsic:
     software_emulation_of_raw_intrinsic_function(a, b)
  }
}

#[ifunc("default", "avx")]
fn my_intrinsic_rt(a: f64, b: f64) -> f64 { my_intrinsic(a, b) }
```
Due to the low-level and high-level nature of these feature we will need two
kinds of documentation. For the low level part:

- document how to do compile-time and run-time feature detection using `cfg!(target_feature)` and `cfg_feature_enabled!`,
- document how to use `#[target_feature]`,
- document how to use all of these together to solve problems like in the examples of this RFC.

For the high-level part we should aim to bring third-party crates implementing
`ifunc!` or similar close to 1.0 releases before stabilization.

# Drawbacks
[drawbacks]: #drawbacks

- Obvious increase in language complexity.

The main drawback of not solving this issue is that many libraries that require
conditional feature-dependent compilation or run-time selection of code for
different features (SIMD, BMI, AES, ...) cannot be written efficiently in stable
Rust.


# Alternatives
[alternatives]: #alternatives

# Backend options

An alternative would be to mix stable, unstable, unknown,
and backend-specific features into `--target-feature`.

## Make `#[target_feature]` safe

Calling a function annotated with `#[target_feature]` on a host that does not
support the feature invokes undefined behavior in LLVM, the assembler, and
possibly the hardware [See this comment](https://github.com/rust-lang/rfcs/pull/2045#issuecomment-311325202).

That is, calling a function on a target that does not support its feature set is
_undefined behavior_ and this RFC cannot specify otherwise. The main reason is that `target_feature` is a promise from the user to the toolchain and the hardware, that the code will not be reached in a CPU that does not support the feature. LLVM, the assembler, and the hardware all assume that the user will not violate this contract, and there is little that the Rust compiler can do to make this safer:
  - The Rust compiler cannot emit a compile-time diagnostic because it cannot know whether the user is going to run the binary in a CPU that supports the features or not.
  - A run-time diagnostic _always_ incurs a run-time cost, and is only possible iff the absence of a feature can be detected at run-time (the "Future Extensions" section of this RFC discusses how to implement "Run-time diagnostics" to detect this, when possible).

However, the `--target-feature/--target-cpu` compiler options allows one to implicitly generate binaries that reliably run into undefined behavior without needing any `unsafe` annotations at all, so the answer to the question "Should `#[target_feature]` be safe/unsafe?" is indeed a hard one.

The main differences between `#[target_feature]` and `--target-feature`/`--enable-feature` are the following:
- `--target-feature/--enable-feature` are "backend options" while `#[target_feature]` is part of the language
- `--target-feature/--enable-feature` is specified by whoever compiles the code, while `#[target_feature]` is specified by whoever writes the code
- compiling safe Rust code for a particular target, and then running the binary on that target, can only produce undefined behavior iff `#[target_feature]` is safe.

This RFC chooses that the `#[target_feature]` attribute only applies to `unsafe fn`s, so that if one compiles safe Rust source code for a particular target, and then runs the binary on that particular target, no unsafety can result.

Note that we can always make `#[target_feature]` safe in the future without breaking backwards compatibility, but the opposite is not true. That is, if somebody figures out a way of making `#[target_feature]` safe such that the above holds, we can always make that change.

## Guarantee no segfaults from `unsafe` code

Calling a `#[target_feature]`-annotated function on a platform that does not
support it invokes undefined behavior. We could guarantee that this does not
happen by always doing run-time feature detection, introducing a run-time cost
in the process, and by only accepting features for which run-time feature
detection can be done.

This RFC considers that any run-time cost is unacceptable as a default
for a combination of language features whose main domain of use is a performance
sensitive one.

The "Future Extension"s section discusses how to implement this in an opt-in way,
e.g., as a sort of binary instrumentation.

## Make `#[target_feature] + #[inline(always)]` incompatible

This RFC requires the compiler to error when a function marked with both `#[target_feature]` and the `#[inline(always)]` attribute cannot be inlined in a particular call site due to incompatible features. So we might consider to simplify this RFC by just making these attributes incompatible.

While this is technically correct, the compiler must detect when any function (`#[inline(always)]`, `#[inline]`, generics, ...) is inlined into an incompatible context, and prevent this from happening. Erroring if the function is `#[inline(always)]` does not significantly simplify the RFC nor the compiler implementation.

## Removing run-time feature detection from this RFC

This RFC adds an API for run-time feature detection to the
standard library.

The alternative would be to implement similar functionality as a third-party crate that
might eventually be moved into the nursery. [Such crates already exist](https://docs.rs/cupid/)

In particular, the API proposed in this RFC is "stringly-typed" (to make it uniform with the other features being proposed), but arguably a third party crate might want to use an `enum` to allow pattern-matching on features. These APIs have not been sufficiently explored in the ecosystem yet.

The main arguments in favor of including run-time feature detection in this RFC are:

- it is impossible to write safe wrappers around `#[target_feature]` without it
- implementing it requires the `asm!` macro or linking to a C library (or linking
  to a C wrapper around assembly),
- run-time detection should be kept in sync with the addition of new target features,
- the compiler might want to use LLVM's run-time feature detection which is part
  of compiler-rt.

The consensus in the internal forums and previous discussions seem to be that this
is worth it.

It might turn out that the people from the future are able to come up with a better
API. But in that case we can always deprecate the current API and include the new
one in the standard library.

## Adding full cpuid support to the standard library

The `cfg_feature_enable!` macro is designed to work specifically with the features
that can be used via `cfg_target_feature` and `#[target_feature]`. However, in the
grand scheme of things, run-time detection of these features is only a small part
of the information provided by `cpuid`-like CPU instructions.

Currently at least two great implementations of cpuid-like functionality exists in
Rust for x86: [cupid](https://github.com/shepmaster/cupid) and
[rust-cpuid](https://github.com/gz/rust-cpuid). Adding the macro to the standard library
does not prevent us from adding more comprehensive functionality in the future, and
it does not prevent us from reusing any of these libraries in the internal
implementation of the macro.

# Unresolved questions
[unresolved]: #unresolved-questions

## How accurate should cfg!(feature) be?

What happens if the macro `cfg!(target_feature = "feature_name")` is used inside a function for which `feature_name` is not enabled, but that function gets inlined into a context in which the feature is enabled? We want the macro to accurately return `true` in this case, that is, to be as accurate as possible so that users always get the most efficient algorithms, but whether this is even possible is an unresolved question.

This might result in monomorphization errors if `#![cfg(target_feature)]` is used, but not if `if cfg!(target_feature)` is used since in this case all branches need to type-check properly.

We might want to ammend this RFC with more concrete semantics about this as we improve the compiler.

## How do we handle ABI issues with portable vector types?

The ABI of `#[target_feature]` functions does not change for all types currently available in stable Rust. However, there are types that we might want to add to the language at some point, like portable vector types, for which this is not the case.

The behavior of `#[target_feature]` for those types should be specified in the RFC that proposes to stabilize those types, and this RFC should be ammended as necessary.

The following examples showcase some potential problems when calling functions with mismatching ABIs, or when using function pointers.

Whether we can warn, or hard error at compile-time in these cases remains to be explored.

**Example 8 (ABI):**

```rust
#[target_feature(enable = "sse2")]
unsafe fn foo_sse2(a: f32x8) -> f32x8 { a } // ABI: 2x 128bit registers

#[target_feature(enable = "avx2")]
unsafe fn foo_avx2(a: f32x8) -> f32x8 { // ABI: 1x 256bit register
  foo_sse2(a) // ABI mismatch:
  //^ should this perform an implicit conversion, produce a hard error, or just undefined behavior?
}

#[target_feature(enable = "sse2")]
unsafe fn bar() {
  type fn_ptr = fn(f32x8) -> f32x8;
  let mut p0: fn_ptr = foo_sse2; // OK
  let p1: fn_ptr = foo_avx2; // ERROR: mismatching ABI
  let p2 = foo_avx2; // OK
  p0 = p2; // ERROR: mismatching ABI
}
```

# Future Extensions

## Mutually exclusive features

In some cases, e.g., when enabling AVX but disabling SSE4 the compiler should probably produce an error, but for other features like `thumb_mode` the behavior is less clear. These issues should be addressed by the RFC proposing the stabilizaiton of the target features that need them, as future extensions to this RFC.

## Safely inlining `#[target_feature]` functions on more contexts

The problem is the following:

```rust
#[target_feature(enable = "sse3")]
unsafe fn baz() {
    if some_opaque_code() {
        unsafe { foo_avx2(); }
    }
}
```

If `foo_avx2` gets inlined into `baz`, optimizations that reorder its instructions
across the if condition might introduce undefined behavior.

Maybe, one could make `cfg_feature_enabled!` a bit magical, so that when it is
used in the typical ways the compiler can infer whether inlining is safe, e.g.,

```rust
#[target_feature(enable = "sse3")]
unsafe fn baz() {
  // -- sse3 boundary start (applies to fn arguments as well)
  // -- sse3 boundary ends
  if cfg_feature_enabled!("avx") {
    // -- avx boundary starts
    unsafe { foo_avx(); }
    //    can be inlined here, but its code cannot be
    //    reordered out of the avx boundary
    // -- avx boundary ends
  }
  // -- sse3 boundary starts
  // -- sse3 boundary ends (applies to drop as well)
}
```

Whether this is worth it or can be done at all is an unresolved question. This RFC does not propose any of this, but leaves the door open for such an extension to be explored and proposed independently in a follow-up RFC.

## Run-time diagnostics

Calling a `#[target_feature]`-annotated function on a platform that does not
support it invokes undefined behavior. A friendly compiler could use run-time
feature detection to check whether calling the function is safe and emit a nice
`panic!` message.

This can be done, for example, by desugaring this:

```rust
#[target_feature(enable = "avx")] unsafe fn foo();
```

into this:

```rust
#[target_feature(enable = "avx")] unsafe fn foo_impl() { ...foo tokens... };

// this function will be called if avx is not available:
fn foo_fallback() {
    panic!("calling foo() requires a target with avx support")
}

// run-time feature detection on initialization
static foo_ptr: fn() -> () = if cfg_feature_enabled!("avx") {
    unsafe { foo_impl }
} else {
    foo_fallback
};

// dispatches foo via function pointer to produce nice diagnostic
unsafe fn foo() { foo_ptr() }
```

This is not required for safety and can be implemented into the compiler as an opt-in instrumentation pass without
going through the RFC process. However, a proposal to enable this by default should go through the RFC process.

## Disabling features

This RFC does not allow disabling target features, but suggest an analogous syntax to do so (`#[target_feature(disable = "feature-list")]`, `--disable-feature=feature-list`). Disabling features can result in some [non-sensical situations](https://internals.rust-lang.org/t/pre-rfc-stabilization-of-target-feature/5176/26) and should be pursued as a future extension of this RFC once we want to stabilize a target feature for which it makes sense.

# Acknowledgements
[acknowledgments]: #acknowledgements

@parched @burntsushi @alexcrichton @est31 @pedrocr @chandlerc @RalfJung @matthieu-m

- `#[target_feature]` Pull-Request: https://github.com/rust-lang/rust/pull/38079
- `cfg_target_feature` tracking issue: https://github.com/rust-lang/rust/issues/29717
