actor Main
  new create(env: Env) =>
    let x: USize = 123
    let y: USize = 0
    env.out.print("123 / 0 = " + (x / y).string())
    env.out.print("123 / 0 = " + (x /~ y).string())
    try
      env.out.print("123 / 0 = " + (x /? y).string())
    else
      env.out.print("divided by zero ")
    end

    env.out.print("max + 1 = " + (U32.max_value() + 1).string())
