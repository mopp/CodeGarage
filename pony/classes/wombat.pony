class Wombat
  let name: String
  var _hunger_level: U64
  var _thirst_level: U64 = 1

  new create(name': String) =>
    name = name'
    _hunger_level = 0

  new hungry(name': String, hunger': U64) =>
    name = name'
    _hunger_level = hunger'

  fun hunger(): U64 =>
    _hunger_level

  fun ref set_hunger(to: U64 = 0): U64 =>
    _hunger_level = to

  fun string(): String =>
    "The hunger level of Wombat " + name + " is " + _hunger_level.string()

class Hawk
  var _hunger_level: U64 = 0

actor Main
  new create(env: Env) =>
    let defaultWombat = Wombat("Fantastibat") // Invokes the create method by default
    let defaultHawk = Hawk
    env.out.print(defaultWombat.string())

    defaultWombat.set_hunger(100)
    env.out.print(defaultWombat.string())
