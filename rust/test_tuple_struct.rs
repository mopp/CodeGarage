struct Inches(i32);
struct Point(i32, i32, i32);

fn main() {
    let length = Inches(10);

    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

    let p = Point(10, 20, 10);
    let Point(x, y, z) = p;
    println!("({}, {}, {})", x, y, z);
}
