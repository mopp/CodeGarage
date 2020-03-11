actor Main
  new create(env: Env) =>
    var x: (String, U64)
    x = ("hi", 3)
    env.out.print("First: " + x._1)
    env.out.print("Second: " + x._2.string())

    var y: (String | None)
    y = "hey"
    y = None
