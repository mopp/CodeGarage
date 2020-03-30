class Foo
  var _x: U64 = 0

  fun apply() =>
    None

  fun ref update(x': U64, value: U32) =>
    _x = x'

  fun ref x(): U64 =>
    _x

actor Main
  new create(env: Env) =>
    env.out.print("hi")

    // Apply
    let foo = Foo.create()
    foo()
    foo.apply()

    let f = Foo()
    // => Foo.create().apply()

    foo(999) = 10
    // => foo.update(999, where value = x)
    env.out.print(foo.x().string())
