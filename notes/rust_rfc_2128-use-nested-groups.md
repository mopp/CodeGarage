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

構文
```
IMPORT = ATTRS VISIBILITY `use` [`::`] IMPORT_TREE `;`

IMPORT_TREE = `*` |
              REL_MOD_PATH `::` `*` |
              `{` IMPORT_TREE_LIST `}` |
              REL_MOD_PATH `::` `{` IMPORT_TREE_LIST `}` |
              REL_MOD_PATH [`as` IDENT]

IMPORT_TREE_LIST = Ø | (IMPORT_TREE `,`)* IMPORT_TREE [`,`]

REL_MOD_PATH = (IDENT `::`)* IDENT
```

Resolution:

最初に、`::`, `self`, `super`で始まらない限り、インポートツリーの前に`::`が付けられる
次に、`a::b::self`を不正なものとするために`{self}`/`{self as name}`が特別に処理されることを除いて、インポートツリー全体が平坦化されたように処理される

```rust
// 変換前
use a::{
    b::{self as s, c, d as e},
    f::*,
    g::h as i,
    *,
};

// 変換後
use ::a::b as s;
use ::a::b::c;
use ::a::b::d as e;
use ::a::f::*;
use ::a::g::h as i;
use ::a::*;
```

このデシュガーにより、いろいろなコーナーケースが自然に解決される

```
use an::{*, *}; // Use an owl!

=>

use an::*;
use an::*; // Legal, but reported as unused by `unused_imports` lint.
```



## 他の提案との関係
このRFCは他のインポートに関する提案とは独立した、インクリメンタルな改善であるが、その他のRFCに影響を持ちうる.
This RFC is an incremental improvement largely independent from other import-related proposals, but it can have effect on some other RFCs.

いくつかのRFCでは現在クレートの絶対パスと他のクレートからのパスのための新しい構文を提案している.  
Some RFCs propose new syntaxes for absolute paths in the current crate and paths from other crates.

それらの提案でのいくつかの議論は使用統計(他のクレートからのインポートがより一般的、なのか、現在のクレートからのインポートがより一般的なのか)にもとづいている.  
より一般的なインポートがうるさくない(verboseではない)構文のほうがよい
> 使用統計が謎い ("imports from other crates are more common" or "imports from the current crate are more common". )

共通のプレフィックスを持つ全てのインポートを冗長で無くすることで、このRFCではそれら統計をthe equationから削除した
> This RFC removes these statistics from the equation by reducing verbosity for all imports with common prefix.

例として、`A`, `B`, `C`の冗長性の差は最小限であり、インポートの数に依存していない.

```rust
// A
use extern::{
    a::b::c,
    d::e::f,
    g::h::i,
};
// B
use crate::{
    a::b::c,
    d::e::f,
    g::h::i,
};
// C
use {
    a::b::c,
    d::e::f,
    g::h::i,
};
```


# Drawbacks
(必須ではないが)、この機能は単一インポートを複数行でフォーマットすることを推奨する

```rust
use prefix::{
    MyName,
    x::YourName,
    y::Surname,
};
```

このフォーマットにすると、`use.*MyName`でgrepするのが難しくなる


# 根拠と代替案
現状維持が常に代替案



# 未解決の問題
今のところ無い
