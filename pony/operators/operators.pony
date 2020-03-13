class Pair
  var _x: U32
  var _y: U32

  new create(x: U32, y: U32) =>
    _x = x
    _y = y

  fun add(other: Pair): Pair =>
    Pair(_x + other._x, _y + other._y)

actor Main
  new create(env: Env) =>
    let x = Pair(1, 2)
    let y = Pair(3, 4)
    let z = x + y

    let a: U32 = 10 % 2
    env.out.print("1 + " + a.string())
