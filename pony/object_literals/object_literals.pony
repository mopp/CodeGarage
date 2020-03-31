use "collections"

class Foo
  fun foo(str: String): Hashable iso^ =>
    object iso is Hashable
      let s: String = str
      fun apply(): String =>
        s
      fun hash(): USize =>
        s.hash()
    end

actor Main
  new create(env: Env) =>
    env.out.print("hi")

    let a = Foo
    let x = a.foo("hello")
    env.out.print(x.hash().string())
