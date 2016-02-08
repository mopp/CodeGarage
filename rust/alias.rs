fn main() {
    {
        let mut data = vec![1, 2, 3];
        let x = &data[0];

        // cannot borrow `data` as mutable because it is also borrowed as immutable
        // "A mutable reference cannot be aliased"というルールがあるので、
        // xとpushメソッドの取るmutalbe refrenceが同時に存在することは許されない.
        data.push(4);

        println!("{}", x);
    }
}
