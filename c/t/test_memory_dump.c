#include "../minunit.h"
#include "../memory_dump.h"


static char const* const str = "ThisIsTestString";


static char const* test_dump(void) {
    dump_memory_hex((uintptr_t)str, 100);

    return NULL;
}


static char const* all_tests(void) {
    MIN_UNIT_RUN(test_dump);
    return NULL;
}


int main(void) {
    MIN_UNIT_RUN_ALL(all_tests);
}
