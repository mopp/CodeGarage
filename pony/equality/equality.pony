class Foo
  let _a: String

  new create(a: String) =>
    _a = a

  fun eq(that: box->Foo): Bool =>
    this._a == that._a


actor Main
  new create(env: Env) =>
    if None is None then
      env.out.print("hi")
    end

    let a = Foo("x")
    let b = Foo("x")

    if a is b then
      env.out.print("nope")
    else
      env.out.print("hi")
    end

    if a == b then
      env.out.print("hi")
    else
      env.out.print("nope")
    end
