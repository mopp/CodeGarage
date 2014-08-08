#include "../lqueue.h"
#include "../macro.h"
#include "../minunit.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>


#define MAX_SIZE 10
static int test_array[MAX_SIZE] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
static char const* test_words[] = {"Apple", "Orange", "Banana", "Lemon", "Lime", "Strawberry"};
#define TEST_WORDS_SIZE (sizeof(test_words) / sizeof(test_words[0]))
static int const check_size = MAX_SIZE;


static void release_str(Dlist* q, void* d) {
    free(d);
}


static char const* test_lqueue(void) {
    Lqueue q;
    Lqueue* const qp = &q;

    lqueue_init(qp, sizeof(int), NULL);

    MIN_UNIT_ASSERT("lqueue_init is wrong.", qp->list != NULL);
    MIN_UNIT_ASSERT("lqueue_init is wrong.", lqueue_get_size(qp) == 0);

    for (int i = 0; i < check_size; i++) {
        lqueue_insert(qp, &test_array[i]);
        MIN_UNIT_ASSERT("lqueue_insert is wrong.", *(int*)lqueue_get_first(qp) == test_array[0]);
    }
    MIN_UNIT_ASSERT("lqueue_insert result is wrong.", lqueue_get_size(qp) == check_size);

    for (int i = 0; i < check_size; i++) {
        int n = *(int*)lqueue_get_first(qp);
        MIN_UNIT_ASSERT("lqueue_get_first is wrong.", n == test_array[i]);
        lqueue_delete_first(qp);
    }
    MIN_UNIT_ASSERT("lqueue_delete_first result is wrong.", lqueue_get_size(qp) == 0);

    lqueue_destruct(qp);

    lqueue_init(qp, sizeof(char*), release_str);
    for (int i = 0; i < TEST_WORDS_SIZE; i++) {
        char* c = (char*)malloc(strlen(test_words[i]));
        strcpy(c, test_words[i]);
        lqueue_insert(qp, &c);
        MIN_UNIT_ASSERT("", strcmp(*(char**)lqueue_get_first(qp), test_words[i]) == 0);
        lqueue_delete_first(qp);
    }
    MIN_UNIT_ASSERT("", lqueue_get_size(qp) == 0);
    lqueue_destruct(qp);
    MIN_UNIT_ASSERT("", lqueue_get_size(qp) == 0);

    return NULL;
}

static char const* all_tests(void) {
    MIN_UNIT_RUN(test_lqueue);

    return NULL;
}

int main(void) {
    MIN_UNIT_RUN_ALL(all_tests);

    return 0;
}
