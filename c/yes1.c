#include <stdio.h>


int main(int argc, char const* const argv[]) {
    for (;;) {
        puts((1 < argc) ? (argv[1]) : ("yes"));
    }

    return 0;
}
