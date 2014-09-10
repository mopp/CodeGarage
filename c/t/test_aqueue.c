#include "../minunit.h"
#include "../aqueue.h"
#include "../macro.h"
#include <string.h>
#include <stdlib.h>


#define MAX_CAPACITY 10
static char const* sample_str_data[] = {"Apple", "Orange", "Banana", "Lemon", "Lime", "Strawberry"};
#define STR_NUM ARRAY_SIZE_OF(sample_str_data)


static void release_str(void* d) {
    /*
     * NOTE: In this, free(d) must be written
     * Because, elements memory area is allocated in aqueue function.
     * And maybe "d" is const char** so const area is not freed.
     */
}


static char const* test_aqueue(void) {
    Aqueue aq;
    Aqueue* const p = &aq;

    aqueue_init(p, sizeof(int), MAX_CAPACITY, NULL);
    MIN_UNIT_ASSERT("aqueue_init is wrong.", aqueue_is_empty(p) == true);
    MIN_UNIT_ASSERT("aqueue_init is wrong.", aqueue_get_capacity(p) == MAX_CAPACITY);
    MIN_UNIT_ASSERT("aqueue_init is wrong.", aqueue_get_size(p) == 0);

    for (int i = 0; i < MAX_CAPACITY; i++) {
        aqueue_insert(p, &i);
        MIN_UNIT_ASSERT("aqueue_insert is wrong.", aqueue_get(int, p) == 0);
    }
    MIN_UNIT_ASSERT("aqueue_insert result is wrong.", aqueue_get_size(p) == MAX_CAPACITY);

    for (int i = 0; i < MAX_CAPACITY; i++) {
        int* t = (int*)aqueue_get_first(p);
        MIN_UNIT_ASSERT("aqueue_get is wrong.", *t == i);
        aqueue_delete_first(p);
    }
    MIN_UNIT_ASSERT("aqueue_delete_first is wrong", aqueue_is_empty(p) == true);
    aqueue_destruct(p);

    aqueue_init(p, sizeof(char*), MAX_CAPACITY, release_str);
    for (int i = 0; i < STR_NUM; i++) {
        aqueue_insert(p, &sample_str_data[i]);
        MIN_UNIT_ASSERT("aqueue_insert is wrong", 0 == strcmp(sample_str_data[0], aqueue_get(char const*, p)));
    }
    MIN_UNIT_ASSERT("aqueue_insert result is wrong.", aqueue_get_size(p) == STR_NUM);
    MIN_UNIT_ASSERT("aqueue_insert result is wrong.", aqueue_is_empty(p) == false);

    aqueue_destruct(p);
    MIN_UNIT_ASSERT("aqueue_destruct is wrong.", p->data != NULL);

    printf("All Test Passed.\n");

    return NULL;
}


static char const* all_tests(void) {
    MIN_UNIT_RUN(test_aqueue);
    return NULL;
}


int main(void) {
    MIN_UNIT_RUN_ALL(all_tests);
}
