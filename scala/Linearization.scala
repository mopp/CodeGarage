trait TraitA {
  def greet(): Unit = println("Hello !")
}


trait TraitB extends TraitA {
  override def greet(): Unit = {
    super.greet()
    println("Good morning !")
  }
}


trait TraitC extends TraitA {
  override def greet(): Unit = {
    super.greet()
    println("Good evening !")
  }
}

// Stackable Trait
class ClassA extends TraitB with TraitC
class ClassB extends TraitC with TraitB


object Linearization {
  def main(args: Array[String]) {
    val classA = new ClassA()
    classA.greet();

    val classB = new ClassB()
    classB.greet();
  }
}
