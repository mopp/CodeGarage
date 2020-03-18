actor Main
  new create(env: Env) =>
    let x: USize = 1 + if true then 1 else 0 end
    env.out.print(x.string())

    // Union.
    let friendly = false
    let y: (String | Bool) =
      if friendly then
        "Hello"
      else
        false
      end
    env.out.print(y.string())

    let z: (String | None) =
      if friendly then
        "Hello"
      end
    env.out.print(z.string())

    // for with else
    let a: (String | None) =
      for name in Array[String].values() do
        name
      else
        "no names!"
      end

    match a
    | let s: String => env.out.print("x is " + s)
    | None => env.out.print("x is None")
    end

    let b =
      while false do
        "hi"
      else
        "wow"
      end
    env.out.print(b.string())
