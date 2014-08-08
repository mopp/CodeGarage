#include "minunit.h"


static int foo = 7;
static int bar = 5;


static char const *test_foo() {
    MIN_UNIT_ASSERT("ERROR: foo != 7", foo == 7);
    return NULL;
}


static char const *test_bar() {
    MIN_UNIT_ASSERT("ERROR: bar != 5", bar == 5);
    return NULL;
}


static char const *all_tests() {
    MIN_UNIT_RUN(test_foo);
    MIN_UNIT_RUN(test_bar);
    return NULL;
}


int main(void) {
    MIN_UNIT_RUN_ALL(all_tests);
}
