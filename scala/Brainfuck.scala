import scala.util.control.Breaks.{break, breakable}

object Brainfuck {
  private class Executor(memorySize: Int = 30000) {
    private var memory: Array[Byte] = Array.fill(memorySize)(0);
    private var ptr = 0;

    def input(in: String) {
      println("Execute: " + in);
      var len = in.length;

      var i = 0;
      while (i < len) {
        in(i) match {
          case '>' => ptr += 1;
          case '<' => ptr -= 1;
          case '+' => memory(ptr) = memory(ptr).+(1).toByte;
          case '-' => memory(ptr) = memory(ptr).-(1).toByte;
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
  }

  def main(args: Array[String]) {
    var exe = new Executor();
    exe.input("+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.");
  }
}
