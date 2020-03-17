actor Main
  new create(env: Env) =>
    let x: USize = 1 + if true then 1 else 0 end
    env.out.print(x.string())
