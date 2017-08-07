# Rustのカスタムアロケータについて



## [rfcs/1183-swap-out-jemalloc.md at 2a5c043e06688a6923102e290b91faf5856649db · rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/2a5c043e06688a6923102e290b91faf5856649db/text/1183-swap-out-jemalloc.md)
古いカスタムアロケータ  
開始日時は2015-06-27  
**2017-01-18に新しいものに取って代わられた**  
古いものなので詳解はしない  

クレートに`#![allocator]`を付けて以下の関数群を実装する必要がある.
```rust
extern {
    fn __rust_allocate(size: usize, align: usize) -> *mut u8;
    fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize);
    fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                         align: usize) -> *mut u8;
    fn __rust_reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize,
                                 align: usize) -> usize;
    fn __rust_usable_size(size: usize, align: usize) -> usize;
}
```


## [rfcs/1974-global-allocators.md at 647378d9ff752f95dbf9169f55d37234360087c7 · rust-lang/rfcs](https://github.com/rust-lang/rfcs/blob/647378d9ff752f95dbf9169f55d37234360087c7/text/1974-global-allocators.md)
新しいカスタムアロケータ  
開始日時は2017-02-04  
Global Allocatorと呼ばれているらしい
詳細は調べていないが古いカスタムアロケータ(RFC 1183)のrefine版のよう  

古いカスタムアロケータには2つ問題があった
    * 古いCスタイルのAPIはエラーが発生しやすい
        * 関数シグネチャが正しいかどうかの保証がない
        * 関数が存在しなかった場合、コンパイラではなくリンカーがエラーを出す
    * アロケータが状態を持つ
        * `bare function`は状態を保持できないので、その状態が真にグローバルであることが強要される
また、アロケータをクレートととして追加すると自動で使用されるようになるので複数のアロケータを組み合わせるのが難しい

`alloc_system`も非推奨になり`SystemAllocator`構造体で置き換えられる予定らしい
