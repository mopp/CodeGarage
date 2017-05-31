// https://doc.rust-lang.org/std/mem/index.html
use std::{mem, ptr};


fn main()
{
    uninitialized_memory();
    align_of_memory();
    dispose_value();
    forget_value();
}


fn uninitialized_memory()
{
    let mut data: [usize; 1000];

    unsafe {
        data = mem::uninitialized();

        for elem in &mut data[..] {
            ptr::write(elem, 100usize);
        }
    }

    for elem in &mut data[..] {
        assert_eq!(*elem, 100);
    }
}


fn align_of_memory()
{
    assert_eq!(4, mem::align_of::<i32>());
    assert_eq!(8, mem::align_of::<i64>());

    struct ObjA {
        i: i32
    }

    struct ObjB {
        i: i32,
        j: i32
    }

    struct ObjC {
        i: u8
    }

    struct ObjD {
        i: u8,
        j: u8
    }

    struct ObjE {
        i: u8,
        j: u8,
        k: i32,
    }

    struct ObjF {
        i: u8,
        j: u8,
        k: u64,
    }

    assert_eq!(4, mem::align_of::<ObjA>());
    assert_eq!(4, mem::align_of::<ObjB>());
    assert_eq!(1, mem::align_of::<ObjC>());
    assert_eq!(1, mem::align_of::<ObjD>());
    assert_eq!(4, mem::align_of::<ObjE>());
    assert_eq!(8, mem::align_of::<ObjF>());
}


fn dispose_value()
{
    let v = vec![1, 2, 3];
    drop(v);

    // Error: value used here after move.
    // println!("{:?}", v);

    let mut v = vec![1, 2, 3];
    let x = &v[0];

    // explicitly drop the reference, but the borrow still exists
    // borrowの先は解放されない
    drop(x);

    // v.push(4); // error: cannot borrow `v` as mutable because it is also borrowed as immutable
}


// * 値をリークさせる
//     * デストラクタを実行せずに所有権を取って、忘れる
//     * 例えばヒープやファイルハンドラなど、到達されない状態で永久に残る
//     * もし、正確に破棄したいのであれば、`mem::drop`を使用する
// * 安全性
//     * Rustはデストラクタが必ず実行される保証をしないので、`forget`はunsafeではない
fn forget_value()
{
    // 初期化されていない値のデストラクタを呼ぶことは未定義動作を引き起こす
    // なので、以下のとき、もし初期化できなかった場合は、デストラクタを呼ばないように破棄する
    unsafe {
        let mut uninit_vec: Vec<u32> = mem::uninitialized();
        let some_condition = false;

        if some_condition {
            ptr::write(&mut uninit_vec, Vec::new());
        } else {
            mem::forget(uninit_vec);
        }
    }


    let x = &mut 100;
    let y = &mut 999;
    println!("x = {}, y = {}", x, y);
    unsafe {
        // スワップ用のメモリ領域を確保
        let mut t: usize = mem::uninitialized();

        // スワップ
        ptr::copy_nonoverlapping(&*x, &mut t, 1);
        ptr::copy_nonoverlapping(&*y, x, 1);
        ptr::copy_nonoverlapping(&t, y, 1);

        // ここで`y`と`t`は同じ領域を指している
        // なので、`t`のデストラクタが呼ばれ、領域が破棄されると困るので`forget`を使う
        // TODO: 破棄される例
        mem::forget(t);
    }
    println!("x = {}, y = {}", x, y);
}


// * 値の置き換えを、対象2つのデストラクタを実行することなく行う
//     * 古い値は戻り値として返る
fn replace_value()
{
    let mut v: Vec<i32> = vec![1, 2];

    let old_v = mem::replace(&mut v, vec![3, 4, 5]);
    assert_eq!(2, old_v.len());
    assert_eq!(3, v.len());

    // 構造体のフィールド入れ替えで有用
    // またTは`Clone`を実装している必要はない
    struct Buffer<T> { buf: Vec<T> }

    impl<T> Buffer<T> {
        fn get_and_reset(&mut self) -> Vec<T> {
            // error: cannot move out of dereference of `&mut`-pointer
            // let buf = self.buf;
            // self.buf = Vec::new();
            // buf

            mem::replace(&mut self.buf, Vec::new())
        }
    }
}
