macro_rules! interpret_basic {
    () => {
        ()
    };

    (PRINT ; $( $rest:tt )*) => {
        println!("");
        interpret_basic!($( $rest )*);
    };

    (PRINT $( $args:expr ),+ ; $( $rest:tt )*) => {
        $( print!("{}", $args); )+
        interpret_basic!($( $rest )*);
    };

    (INPUT $var:ident ; $( $rest:tt )*) => {
        let mut $var: usize = {
            // https://github.com/rust-lang/rust/issues/23818
            use std::io::Write;
            std::io::stdout().flush().expect("could not flush the stdout.");

            let mut number_string = String::new();
            std::io::stdin()
                .read_line(&mut number_string)
                .expect("Failed to read line");

            number_string
                .trim()
                .parse::<usize>()
                .expect("The input allows only a number.")
        };
        interpret_basic!($( $rest )*);
    };

    (LET_MUT $var:ident = $val:expr ; $( $rest:tt )*) => {
        let mut $var: usize = $val;
        interpret_basic!($( $rest )*);
    };

    (LET $var:ident = $val:expr ; $( $rest:tt )*) => {
        let $var: usize = $val;
        interpret_basic!($( $rest )*);
    };

    (IF ( $cond:expr ) { $( $inner_if:tt )* } ELSE { $( $inner_else:tt )* } $( $rest:tt )*) => {
        if $cond {
            interpret_basic!($( $inner_if )*);
        } else {
            interpret_basic!($( $inner_else )*);
        }
        interpret_basic!($( $rest )*);
    };

    (IF ( $cond:expr ) { $( $inner:tt )* } $( $rest:tt )*) => {
        if $cond {
            interpret_basic!($( $inner )*);
        }
        interpret_basic!($( $rest )*);
    };

    (FOR $var:pat = $begin:tt TO $end:tt { $( $inner:tt )* } $( $rest:tt )*) => {
        for $var in $begin..$end {
            interpret_basic!($( $inner )*);
        }
        interpret_basic!($( $rest )*);
    };

    ($var:ident = $val:expr ; $( $rest:tt )*) => {
        $var = $val;
        interpret_basic!($( $rest )*);
    };

    (FN $name:ident ( $( $args:ident ),* ) { $( $inner:tt )* } $( $rest:tt )*) => {
        let $name = |$( $args: usize ),*| {
            interpret_basic!($( $inner )*);
        };
        interpret_basic!($( $rest )*);
    };

    (RETURN $value:expr; $( $rest:tt )*) => {
        return $value;
    };
}


#[allow(non_snake_case)]
fn main()
{
    interpret_basic!{
        PRINT "Hello, world!\n";

        LET A = 1 + 1;
        LET B = A + 2;
        PRINT "A = ", A, ", ";
        PRINT "B = ", B;
        PRINT ;

        IF (A == 2) {
            PRINT "The condition is matched !\n";
            PRINT "A is 2\n";
        }

        FOR A = 1 TO 10 {
            PRINT A, ", ";
        }
        PRINT;

        PRINT "B = ";
        INPUT B;

        PRINT "Your input number is ";
        PRINT B;
        PRINT "\n";

        IF ((B % 15) == 0) {
            PRINT "FizzBuzz\n";
        } ELSE {
            IF ((B % 3) == 0) {
                PRINT "Fizz\n";
            } ELSE {
                IF ((B % 5) == 0) {
                    PRINT "Buzz\n";
                } ELSE {
                    PRINT B;
                    PRINT "\n";
                }
            }
        }

        B = 100;
        PRINT B;
        PRINT "\nFIN\n";
    };
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    #[test]
    fn test_empty()
    {
        assert_eq!(interpret_basic!{}, ());
    }

    #[test]
    fn test_let()
    {
        interpret_basic!{
            LET A = 100;
            LET longlong_var = 1231;
            LET VAR = 1 + 2 + 3 / 2;
            LET VAR2 = 10 * 10;
        };

        assert_eq!(A, 100);
        assert_eq!(longlong_var, 1231);
        assert_eq!(VAR, 4);
        assert_eq!(VAR2, 100);
    }

    #[test]
    fn test_let_mut()
    {
        interpret_basic!{
            LET_MUT A = 100;
        };

        assert_eq!(A, 100);

        interpret_basic!{
            A = 200;
        };

        assert_eq!(A, 200);
    }

    #[test]
    fn test_if()
    {
        interpret_basic!{
            LET_MUT A = 100;
            IF (A == 100) {
                A = A - 10;
            }
        };

        assert_eq!(A, 90);

        interpret_basic!{
            LET_MUT A = 100;
            IF ((A % 10) == 0) {
                A = A + 10;
            }
        };

        assert_eq!(A, 110);
    }

    #[test]
    fn test_if_else()
    {
        interpret_basic!{
            LET_MUT A = 100;
            IF (A != 100) {
                A = A - 10;
            } ELSE {
                A = A / 10;
            }
        };

        assert_eq!(A, 10);
    }


    #[test]
    fn test_for() {
        interpret_basic!{
            LET_MUT A = 0;
            FOR i = 1 TO 11 {
                A = A + i;
            }
        };

        assert_eq!(A, 55);

        interpret_basic!{
            LET_MUT A = 0;
            FOR i = 1 TO 11 {
                IF ((i % 2) == 0) {
                    A = A + i;
                }
            }
        };
        assert_eq!(A, 2 + 4 + 6 + 8 + 10);
    }

    #[test]
    #[allow(unreachable_code)]
    fn test_fn() {
        interpret_basic!{
            FN SAMPLE_FUNC1() {
            }

            FN SAMPLE_FUNC2() {
                LET A = 1;
                PRINT A;
            }

            FN SAMPLE_FUNC3(A) {
                RETURN A;
            }

            FN SAMPLE_FUNC4(A) {
                RETURN A * 2;
            }

            FN SAMPLE_FUNC5(N) {
                IF ((N % 2) == 0) {
                    RETURN 1;
                } ELSE {
                    RETURN 0;
                }
            }
        };

        assert_eq!(SAMPLE_FUNC1(), ());
        assert_eq!(SAMPLE_FUNC2(), ());
        assert_eq!(SAMPLE_FUNC3(100), 100);
        assert_eq!(SAMPLE_FUNC4(2), 4);
        assert_eq!(SAMPLE_FUNC5(2), 1);
        assert_eq!(SAMPLE_FUNC5(3), 0);
    }

    #[test]
    fn test_return() {
        interpret_basic!{
            FN SAMPLE_FUNC1() {
                RETURN 100;
            }
            LET A = 456;
            LET B = SAMPLE_FUNC1();

            FN FIBONACCI(N) {
                IF ((N == 0) || (N == 1)) {
                    RETURN N;
                }

                LET_MUT X = 0;
                LET_MUT Y = 1;
                LET_MUT F = 0;
                FOR _ = 2 TO (N + 1) {
                    F = X + Y;
                    X = Y;
                    Y = F;
                }

                RETURN F;
            }

            LET F0  = FIBONACCI(0);
            LET F1  = FIBONACCI(1);
            LET F10 = FIBONACCI(10);
        };

        assert_eq!(A, 456);
        assert_eq!(B, 100);
        assert_eq!(F0, 0);
        assert_eq!(F1, 1);
        assert_eq!(F10, 55);
    }
}
