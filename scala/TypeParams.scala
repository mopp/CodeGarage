class Cell[A](var value: A) {
  def put(newValue: A): Unit = {
    value = newValue
  }

  def get(): A = value
}


class Pair[+A, +B](val a: A, val b: B) {
  override
  def toString(): String = "(" + a + "," + b + ")"
}


class InvariantClass[A](val a: A)


class CovariantClass[+A](val a: A)



object TypeParams {
  def divide(m: Int, n: Int): Pair[Int, Int] = new Pair[Int, Int](m / n, m % n)


  def main(args: Array[String]) {
    val c1 = new Cell[Int](1)
    c1.put(2)
    val v = c1.get()

    println(v)

    val r = divide(7, 3)
    println(r)

    var p1: Pair [AnyRef, AnyRef] = new Pair(1: java.lang.Integer, 2: java.lang.Integer);
    val p2 = new Pair[String, String]("foo", "bar")
    p1 = p2;

    println(p1);



    // Invariant
    var invariant_a = new InvariantClass(1)
    var invariant_b = new InvariantClass(2)
    invariant_a = invariant_b

    // Error: type mismatch
    // invariant_a = new InvariantClass("String")



    // Covariant
    var covariant_a: CovariantClass[Int] = new CovariantClass(1)
    var covariant_b: CovariantClass[Any] = covariant_a
    // Error:
    // var invariant_c: InvariantClass[Any] = invariant_a

    // Contravariant
    var contravariant_a: (String) => String = (str: String) => {str}
    println(contravariant_a("String"))
    contravariant_a = (obj: Object) => {"This is an object"}
    println(contravariant_a(null))
  }
}
