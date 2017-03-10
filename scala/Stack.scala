trait Stack[+A] {
  def push[E >: A](e: E): Stack[E]
  def top: A
  def pop: Stack[A]
  def isEmpty: Boolean
}


class NonEmptyStack[+A](private val first: A, private val rest: Stack[A]) extends Stack[A] {
  def push[E >: A](e: E): Stack[E] = new NonEmptyStack[E](e, this)
  def top: A = first
  def pop: Stack[A] = rest
  def isEmpty: Boolean = false
}


case object EmptyStack extends Stack[Nothing] {
  def push[E >: Nothing](e: E): Stack[E] = new NonEmptyStack[E](e, this)
  def top: Nothing = throw new IllegalArgumentException("empty stack")
  def pop: Nothing = throw new IllegalArgumentException("empty stack")
  def isEmpty: Boolean = true
}


object Stack {
  def apply(): Stack[Nothing] = EmptyStack
}


object StackTest {
  def main(args: Array[String]) {
    val intStack: Stack[Int] = Stack()

    val t = intStack.push(1)
    println(t.top)
    println(t.pop.isEmpty)

    val stringStack: Stack[String] = Stack()
  }
}
