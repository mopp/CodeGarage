trait Named
  fun name(): String =>
    // Default implementation
    "Bob"

trait Bald is Named
  fun hair(): Bool =>
    false

// Nominal subtyping
class Bob is Bald

class Larry is HasName

interface HasName
  fun name(): String =>
    "hi"

actor Main
  new create(env: Env) =>
    let b = Bob
    let l = Larry

    env.out.print(b.name())
    env.out.print(l.name())
