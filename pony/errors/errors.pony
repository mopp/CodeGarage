actor Main
  new create(env: Env) =>
    try
      env.out.print("a")

      if not f() then
        env.out.print("cause error")
        error
      end
      env.out.print("can you see?")
    else
      env.out.print("else")
    then
      env.out.print("then")
    end

  fun f(): Bool =>
    false

  fun factorial(x: I32): I32 ? =>
    if x < 0 then
      error
    end

    if x == 0 then
      1
    else
      x * factorial(x - 1)?
    end
