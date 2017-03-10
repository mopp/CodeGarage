class SampleCompanion(val x: Int, private val y: Int, private[this] val z: Int)


object SampleCompanion {
  def print_elements(): Unit = {
    val vector = new SampleCompanion(10, 20, 30)
    println(vector.x)
    println(vector.y)
    // println(vector.z) // error
  }
}


object Companion {
  def main(args: Array[String]): Unit = {
    SampleCompanion.print_elements()
  }
}
