import scala.util.control.Breaks.{break, breakable}

object Brainfuck {
  private class Executor(memorySize: Int = 30000) {
    private var memory: Array[Int] = Array.fill(memorySize)(0);
    private var ptr = 0;

    def exe(in: String) = {
      var len = in.length;

      var i = 0;
      while (i < len) {
        in(i) match {
          case '>' => ptr += 1;
          case '<' => ptr -= 1;
          case '+' => memory(ptr) += 1;
          case '-' => memory(ptr) -= 1;
          case '.' => print(memory(ptr).toChar);
          case ',' => memory(ptr) = Console.in.read.toChar.toByte;
          case '[' => {
            if(memory(ptr) == 0) {
              breakable {
                while (i < len) {
                  i += 1;
                  if(in(i) == ']') {
                    break;
                  }
                }
              }
            }
          }
          case ']' => {
            if(memory(ptr) != 0) {
              breakable {
                while (0 < i) {
                  i -= 1;
                  if(in(i) == '[') {
                    break;
                  }
                }
              }
            }
          }
        }
        i += 1;
      }

      println();
    }

    def exe2(in: String): Boolean = {
      if (in.isEmpty) {
        println();
        return false;
      }

      val c = in.head;
      var next = in.tail;
      if(c == '[' && memory(ptr) == 0) {
        next = in.substring(next.indexOf(']') + 1, in.length);
      } else if(c == ']' && memory(ptr) != 0) {
        return true;
      } else {
        c match {
          case '>' => ptr += 1;
          case '<' => ptr -= 1;
          case '+' => memory(ptr) += 1;
          case '-' => memory(ptr) -= 1;
          case '.' => print(memory(ptr).toChar);
          case ',' => memory(ptr) = Console.in.read.toChar.toByte;
          case _   => ;
        }
      }

      var isLoop = false;
      do {
        isLoop = exe2(next);
      } while(isLoop && c == '[')

      return isLoop;
    }

    def clear() = {
      for (i <- 0 to memorySize - 1) {
        memory(i) = 0;
      }
    }
  }

  def main(args: Array[String]) {
    var code = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";
    println("Execute: " + code);
    var exe = new Executor();
    exe.exe(code);
    exe.clear();
    exe.exe2(code);
  }
}
