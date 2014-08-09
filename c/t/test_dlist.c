#include "../minunit.h"
#include "../dlist.h"
#include <assert.h>
#include <string.h>
#include <stdlib.h>


static char const* test_list_create_destruct() {
    Dlist l;

    dlist_init(&l, sizeof(int), NULL);

    int i = 0;
    dlist_insert_data_last(&l, &i);
    MIN_UNIT_ASSERT("Node is NULL.", l.node != NULL);

    dlist_destruct(&l);
    MIN_UNIT_ASSERT("dlist_destruct is wrong.", l.node == NULL);

    dlist_init(&l, sizeof(long long), NULL);

    long long ll = 0;
    dlist_insert_data_last(&l, &ll);
    MIN_UNIT_ASSERT("Node is NULL.", l.node != NULL);

    dlist_destruct(&l);
    MIN_UNIT_ASSERT("dlist_destruct is wrong.", l.node == NULL);

    return NULL;
}


static inline bool echo_int(void* d) {
    printf("%02d -> ", *(int*)d);
    return false;
}


#define MAX_SIZE 10
static int sample_int_data[MAX_SIZE] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};


static bool for_each_int(Dlist* l, void* d) {
    static int cnt = 0;

    echo_int(d);

    if (dlist_cast_data(int, d) != sample_int_data[MAX_SIZE - cnt++ % MAX_SIZE - 1]) {
        puts("\n");
        return true;
    }

    return false;
}


static bool for_each_int_inv(Dlist* l, void* d) {
    static size_t cnt = 0;

    echo_int(d);
    if (dlist_cast_data(int, d) != sample_int_data[cnt++ % MAX_SIZE]) {
        puts("\n");
        return true;
    }

    return false;
}


static char const* test_int_list(void) {
    Dlist l;
    Dlist_node* n;

    /* Insert first */
    dlist_init(&l, sizeof(int), NULL);

    for (int i = 0; i < MAX_SIZE; i++) {
        dlist_insert_data_first(&l, &sample_int_data[i]);
        MIN_UNIT_ASSERT("dlist_insert_data_first is wrong.", sample_int_data[i] == dlist_get_data(int, l.node));
    }
    MIN_UNIT_ASSERT("list element size is wrong.", dlist_get_size(&l) == MAX_SIZE);

    printf("first -> ");
    n = dlist_for_each(&l, for_each_int, false);
    MIN_UNIT_ASSERT("element order is wrong.", n == NULL);
    puts("last");

    printf("last  -> ");
    n = dlist_for_each(&l, for_each_int_inv, true);
    MIN_UNIT_ASSERT("element order is wrong.", n == NULL);
    puts("first");


    /* Insert last */
    dlist_init(&l, sizeof(int), NULL);
    for (int i = 0; i < MAX_SIZE; i++) {
        dlist_insert_data_last(&l, &sample_int_data[i]);
        MIN_UNIT_ASSERT("dlist_insert_data_first is wrong.", sample_int_data[i] == dlist_get_data(int, l.node->prev));
    }
    MIN_UNIT_ASSERT("list element size is wrong.", dlist_get_size(&l) == MAX_SIZE);

    printf("first -> ");
    n = dlist_for_each(&l, for_each_int_inv, false);
    MIN_UNIT_ASSERT("element order is wrong.", n == NULL);
    puts("last");

    printf("last  -> ");
    n = dlist_for_each(&l, for_each_int, true);
    MIN_UNIT_ASSERT("element order is wrong.", n == NULL);
    puts("first");

    dlist_destruct(&l);

    return NULL;
}


static char const* test_list_manip(void) {
    Dlist l;

    dlist_init(&l, sizeof(long), NULL);

    /* Insert first/next/prev */
    long a = 0xFFFF, b = 0xAFAF, c = 0x2525;
    dlist_insert_data_first(&l, &a);
    assert(dlist_get_data(long, l.node) == a);

    dlist_insert_data_next(&l, l.node, &b);
    assert(dlist_get_data(long, l.node->next) == b);

    dlist_insert_data_prev(&l, l.node, &c);
    assert(dlist_get_data(long, l.node->prev) == c);

    /* Search node */
    Dlist_node* n;
    n = dlist_search_node(&l, &a);
    MIN_UNIT_ASSERT("dlist_search_node is wrong.", n != NULL);
    MIN_UNIT_ASSERT("dlist_search_node is wrong.", dlist_get_data(long, n) == a);
    n = dlist_search_node(&l, &b);
    MIN_UNIT_ASSERT("dlist_search_node is wrong.", n != NULL);
    MIN_UNIT_ASSERT("dlist_search_node is wrong.", dlist_get_data(long, n) == b);
    n = dlist_search_node(&l, &c);
    MIN_UNIT_ASSERT("dlist_search_node is wrong.", n != NULL);
    MIN_UNIT_ASSERT("dlist_search_node is wrong.", dlist_get_data(long, n) == c);

    /* Delete node */

    dlist_delete_node(&l, l.node);
    MIN_UNIT_ASSERT("dlist_delete_node is wrong.", dlist_get_data(long, l.node) == b);

    dlist_delete_node(&l, l.node);
    MIN_UNIT_ASSERT("dlist_delete_node is wrong.", dlist_get_data(long, l.node) == c);

    dlist_delete_node(&l, l.node);
    MIN_UNIT_ASSERT("dlist_delete_node is wrong.", (dlist_get_size(&l) == 0) && (l.node == NULL));

    dlist_destruct(&l);

    return NULL;
}


static char const* sample_str_data[] = {"Apple", "Orange", "Banana", "Lemon", "Lime", "Strawberry"};
#define STR_NUM ARRAY_SIZE_OF(sample_str_data)


static inline bool echo_str(void* d) {
    printf("%s -> ", dlist_cast_data(char const*, d));
    return false;
}


static bool for_each_str(Dlist* l, void* d) {
    static size_t cnt = 0;

    echo_str(d);
    if (dlist_cast_data(char const*, d) != sample_str_data[cnt++]) {
        puts("\n");
        return true;
    }
    return false;
}


static void release_str(Dlist* l, void* d) {
    static size_t cnt = 0;
    assert(strcmp(sample_str_data[cnt++], dlist_cast_data(char const*, d)) == 0);

    free(d);
}


static char const* test_pointer_list(void) {
    Dlist l;

    dlist_init(&l, sizeof(char*), release_str);
    for (int i = 0; i < STR_NUM; i++) {
        /* NOTE: sample_str_data[i] is char "const" *, NOT char* */
        dlist_insert_data_last(&l, &sample_str_data[i]);
    }

    printf("first -> ");
    dlist_for_each(&l, for_each_str, false);
    puts("last");
    MIN_UNIT_ASSERT("list element num is wrong", dlist_get_size(&l) == STR_NUM);

    dlist_destruct(&l);

    return NULL;
}


static void release_str_no_assert(Dlist* l, void* d) {
    free(d);
}


static bool for_each_str_no_assert(Dlist* l, void* d) {
    echo_str(d);
    return false;
}


static char const* test_swap(void) {
    Dlist l;

    dlist_init(&l, sizeof(char*), release_str_no_assert);
    for (int i = 0; i < STR_NUM; i++) {
        /* NOTE: sample_str_data[i] is char "const" *, NOT char* */
        dlist_insert_data_last(&l, &sample_str_data[i]);
    }

    printf("Before swap - first -> ");
    dlist_for_each(&l, for_each_str_no_assert, false);
    puts("last");
    MIN_UNIT_ASSERT("list element num is wrong", dlist_get_size(&l) == STR_NUM);


    void const * t = l.node->data;
    dlist_swap_data(l.node, l.node->prev);
    MIN_UNIT_ASSERT("dlist_swap_data is wrong.", t == l.node->prev->data);
    t = l.node->next->data;
    dlist_swap_data(l.node->next, l.node->prev->prev->prev);
    MIN_UNIT_ASSERT("dlist_swap_data is wrong.", t == l.node->prev->prev->prev->data);


    printf("After  swap - first -> ");
    dlist_for_each(&l, for_each_str_no_assert, false);
    puts("last");
    MIN_UNIT_ASSERT("list element num is wrong", dlist_get_size(&l) == STR_NUM);

    dlist_destruct(&l);

    return NULL;
}


static char const* all_tests(void) {
    MIN_UNIT_RUN(test_list_create_destruct);
    MIN_UNIT_RUN(test_int_list);
    MIN_UNIT_RUN(test_list_manip);
    MIN_UNIT_RUN(test_pointer_list);
    MIN_UNIT_RUN(test_swap);

    return NULL;
}


int main(void) {
    MIN_UNIT_RUN_ALL(all_tests);
}
