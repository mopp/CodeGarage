use "collections"

class C
  fun add(x: U32, y: U32): U32 =>
    x + y

class Foo
  var _x: U32

  new create() =>
    _x = 111

  new from_int(x: U32) =>
    _x = x

  fun get_x(): U32 =>
    _x

  new create2(x: U32 = 0, y: U32 = 0) =>
    _x = x + y

  fun f(a: U32 = 1, b: U32 = 2, c: U32 = 3, d: U32 = 4, e: U32 = 5): U32  =>
    0

primitive Printer
  fun print_two_strings(out: StdStream, s1: String, s2: String) =>
    out.>print(s1).>print(s2)
    // Equivalent to:
    out.print(s1)
    out.print(s2)
    out

actor Main
  new create(env: Env) =>
    let a = Foo.create()
    let b = Foo.from_int(3)
    let c = Foo
    let d: Foo = d.create()
    let e = Foo.create2(where y = 4, x = 3)
    let f = e.f(6, 7 where d = 8)
    env.out.print(c.get_x().string())
    env.out.print(e.get_x().string())

    let list_of_numbers = List[U32].from([1; 2; 3; 4])
    let is_odd = {(n: U32): Bool => (n % 2) == 1}

    for odd_number in list_of_numbers.filter(is_odd).values() do
      env.out.print(odd_number.string())
    end
