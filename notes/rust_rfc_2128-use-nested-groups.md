[rfcs/2128-use-nested-groups.md at master · rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/master/text/2128-use-nested-groups.md)

# まとめ

* インポートでネストされた`{}`グループを使えるようになる
* インポートのネストされた`{}`グループで`*`が使えるようになる

具体例
```rust
use syntax::{
    tokenstream::TokenTree, // >1 segments
    ext::base::{ExtCtxt, MacResult, DummyResult, MacEager}, // nested braces
    ext::build::AstBuilder,
    ext::quote::rt::Span,
};

use syntax::ast::{self, *}; // * in braces

use rustc::mir::{*, transform::{MirPass, MirSource}}; // both * and nested braces
```



# 動機
便利だから

同じクレートからたくさんインポートするときは特にだが、インポートのプレフィックスは共通部分が多くなりがち.  
このネスト可能なグルーピングを使うことで、共通のプレフィックスをまとめて一度に書けるようになる.



# ガイドレベルの説明
共通のプレフィックスをもついくつかの`use`は、プレフィックスを1度だけ書き、全てのサフィックスを`{}`の中に書くことで、`use`一つにまとめることができる  
全種類のサフィックスは(`*`と`{}`を持った"subtrees"も含めて)括弧の中に書くことが出来る

具体例
```rust
// BEFORE
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder,
use syntax::ext::quote::rt::Span,

use syntax::ast;
use syntax::ast::*;

use rustc::mir::*;
use rustc::mir::transform::{MirPass, MirSource};



// AFTER
use syntax::{
    // paths with >1 segments are permitted inside braces
    tokenstream::TokenTree,
    // nested braces are permitted as well
    ext::base::{ExtCtxt, MacResult, DummyResult, MacEager},
    ext::build::AstBuilder,
    ext::quote::rt::Span,
};

// `*` can be listed in braces too
use syntax::ast::{self, *};

// both `*` and nested braces
use rustc::mir::{*, transform::{MirPass, MirSource}};

// the prefix can be empty
use {
    syntax::ast::*;
    rustc::mir::*;
};

// `pub` imports can use this syntax as well
pub use self::Visibility::{self, Public, Inherited};
```

プレフィックスをまとめる`use`は、プレフィックスがまとめられていない`use`と同様な振る舞いをする.



# リファレンスレベルの説明

