actor Main
  new create(env: Env) =>
    var x: String = "hello"
    var x' = x + "world"
    let y: U64 = 123

    if x != x' then
      env.out.print(x)
    end

    var z = "a"
