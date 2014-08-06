#include <stdio.h>
#include "minunit.h"


int foo = 7;
int bar = 4;


static char const *test_foo() {
    MIN_UNIT_ASSERT("ERROR: foo != 7", foo == 7);
    return NULL;
}


static char const *test_bar() {
    MIN_UNIT_ASSERT("ERROR: bar != 5", bar == 5);
    return NULL;
}


static char const *all_tests() {
    MIN_UNIT_RUN_TEST(test_foo);
    MIN_UNIT_RUN_TEST(test_bar);
    return NULL;
}


int main(void) {
    char const *result = all_tests();

    if (result != NULL) {
        printf("%s\n", result);
    } else {
        printf("ALL TESTS PASSED\n");
    }

    printf("The number of test: %d\n", minunit_test_counter);

    return result != 0;
}
