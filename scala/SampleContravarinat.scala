object SampleContravarinat {
  def main(args: Array[String]) {
    // https://github.com/worksap-ate/demo/blob/master/study/programming_in_scala_reading/19.md
    // Function はまさに「戻り値共変/引数反変」。
    // 共変 - 型の特化
    // 反変 - 型の汎化
    class A;
    class B extends A;
    class C extends B;

    def hoge(f: B => B) {}

    hoge((b: B) => new B)
    hoge((b: B) => new C)
    // hoge((b: B) => new A)

    hoge((a: A) => new B)
    hoge((a: A) => new C)

    // hoge((c: C) => new B)
    // hoge((a: C) => new B)

    val factory1 = new Factory("X", new Content());
    val gen1: Content => String = (c) => "Generator 1 " + c.getDescription()
    val gen2: SubContent => String = (c) => "Generator 2 " + c.getDescription()
    println(factory1.generate(gen1));
    println(factory1.generate(gen2));

  }
}

class Content() {
  def getDescription(): String = "Content Class"
}

class SubContent() extends Content {
  def getDescription(): String = "Content Class"
}

class Factory[-A](name: String, obj: Content) {
  val factoryName: String = name
  val target: Content = obj;
  def generate(f: A => String): String = f(obj) + " by x"
}
