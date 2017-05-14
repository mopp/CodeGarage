macro_rules! interpret_basic {
    () => {
        ()
    };

    (PRINT ; $( $rest:tt )*) => {
        println!("");
        interpret_basic!($($rest)*);
    };

    (PRINT $arg:expr ; $( $rest:tt )*) => {
        print!("{}", $arg);
        interpret_basic!($($rest)*);
    };

    (PRINT $arg:expr $(, $args:expr)* ; $( $rest:tt )*) => {
        print!("{}", $arg);
        $(print!("{}", $args))*;
        interpret_basic!($($rest)*);
    };

    (LET_MUT $var:ident = $val:expr ; $( $rest:tt )*) => {
        let mut $var: usize = $val;
        interpret_basic!($($rest)*);
    };

    (LET $var:ident = $val:expr ; $( $rest:tt )*) => {
        let $var: usize = $val;
        interpret_basic!($($rest)*);
    };

    (IF ( $cond:expr ) { $( $inner_if:tt )* } ELSE { $( $inner_else:tt )* } $($rest:tt)*) => {
        if $cond {
            interpret_basic!($($inner_if)*);
        } else {
            interpret_basic!($($inner_else)*);
        }
        interpret_basic!($($rest)*);
    };

    (IF ( $cond:expr ) { $( $inner:tt )* } $($rest:tt)*) => {
        if $cond {
            interpret_basic!($($inner)*);
        }
        interpret_basic!($($rest)*);
    };

    (FOR $var:ident = $begin:tt TO $end:tt { $( $inner:tt )* } $($rest:tt)*) => {
        for $var in $begin..$end {
            interpret_basic!($($inner)*);
        }
        interpret_basic!($($rest)*);
    };

    (INPUT $var:ident ; $($rest:tt)*) => {
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
        interpret_basic!($($rest)*);
    };

    ($var:ident = $val:expr ; $($rest:tt)*) => {
        $var = $val;
        interpret_basic!($($rest)*);
    };
}

#[allow(non_snake_case)]
fn main()
{
    interpret_basic!{
        PRINT "Hello, world!\n";

        LET A = 1 + 1;
        PRINT A, "\n";

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
