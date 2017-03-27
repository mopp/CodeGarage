import scala.io.Source

object HighOrderFunc {
  def withFile[A](filename: String)(f: Source => A): A = {
    val s = Source.fromFile(filename)
    try {
      f(s)
    } finally {
      s.close()
    }
  }


  def printFile(filename: String): Unit = {
    println("==========")
    withFile(filename) { file =>
      file.getLines.foreach(println)
    }
    println("==========")
  }

  def main(args: Array[String]) {
    printFile("HighOrderFunc.scala")
  }
}
