#include "../minunit.h"
#include "../align.h"
#include "../macro.h"


static char const* test(void) {
    uintptr_t inputs[] = {
        0x00000000,
        0x00000001,
        0x00000001,
        0x00000010,
        0x00001001,
        0x00001fff,
        0xc0001f00,
        0x10983102,
    };

    size_t pow[] = {
        1,
        1,
        2,
        16,
        1024 * 4,
        1024 * 4,
        1024 * 4,
        8,
    };

    uintptr_t results[] = {
        0x00000000,
        0x00000001,
        0x00000002,
        0x00000010,
        0x00002000,
        0x00002000,
        0xc0002000,
        0x10983108,
    };

    for (int i = 0; i < ARRAY_SIZE_OF(inputs); i++) {
        uintptr_t t = align_address(inputs[i], pow[i]);
        printf("Input: 0x%zx, Result: 0x%zx, Align: %zu\n", inputs[i], t, pow[i]);
        MIN_UNIT_ASSERT("align_address is wrong.", results[i] == t);
    }

    return NULL;
}


static char const* test2(void) {
    /* printf("Input: 0x%zx, Result: 0x%zx, Align: %zu\n", 100u, align_up(0x45, 0x40), 128u); */
    /* MIN_UNIT_ASSERT("align_address is wrong.", results[i] == t); */

    return NULL;
}


static char const* all_tests(void) {
    MIN_UNIT_RUN(test);
    MIN_UNIT_RUN(test2);
    return NULL;
}


int main(void) {
    MIN_UNIT_RUN_ALL(all_tests);
}
