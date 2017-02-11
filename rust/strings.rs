fn main() {
    // 双方とも中身の文字列はUTF-8.
    // `String`: 文字列
    // `str`: 文字列スライス
    let primitive_string: &str = "`str` is a slice of `u8`.";
    let library_string: String = "`String` is a vector of `u8`.".to_string();

    println!("{}", primitive_string);
    println!("{}", library_string);

    // 普通にダブルクォートしたものは文字列スライスになる.
    let some_str: &str = "something";

    // バイト列アクセスと、文字列のアクセスは違う.
    // -> そもそもバイト列は[u8]を使用するべき.
    println!(".as_bytes()[0]  -> {:?}", some_str.as_bytes()[0]);
    println!(".chars().nth(0) -> {:?}", some_str.chars().nth(0));


    let string: String = "hello".to_string();

    // スライスをベクタに変換して比較.
    let a = string == "hello".to_string();
    // ベクタをの参照を取って、スライスとして比較.
    let b = &string == "hello";
    // PartialEqが定義されているので明示的変換処理は無くても比較できる.
    let c = string == "hello";

    // スライスの作成はコピーが発生しない.
    // -> Stringからstrはコピーが起きないのでコストが低い.
    // Stringの作成はスライスの内容を一旦ベクタに変換する必要がある.
    // -> strからStringの変換はコストが高い.

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    // lifetimeを書くのがめんどい.
    // `name`がスライスなので、参照先がある
    // つまり、どこか別のところに文字列があるということになる.
    // -> 不便
    struct Person_inconvenience<'a> {
        name: &'a str,
    }

    let person = Person_inconvenience { name: "Bob" };

    // `to_string`を書かなくてはならないが
    // `String`は`Vec<u8>`なので、所有権は構造体と共にある.
    struct Person {
        name: String,
    }

    let person = Person { name: "Bob".to_string() };
}
