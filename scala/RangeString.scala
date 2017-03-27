object RangeString {
  def joinByComma(start: Int, end: Int): String = (start to end).mkString(",")

  def main(args: Array[String]) {
    println(joinByComma(1, 10))
    println(joinByComma(3, 9))
  }
}
