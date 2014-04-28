/**
 * @file queue.c
 * @brief queue by list.
 * @author mopp
 * @version 0.1
 * @date 2014-04-24
 */

#include <stdlib.h>
#include <assert.h>
#include <string.h>
#include "queue.h"


Queue* queue_init(Queue* q, size_t size, release_func f, bool is_data_pointer) {
    assert(q != NULL);

    q->list = (List*)malloc(sizeof(List));
    q->is_data_type_pointer = is_data_pointer;

    list_init(q->list, size, f);

    return q;
}


bool queue_is_empty(Queue const* q) {
    assert(q != NULL);

    return (list_get_size(q->list) == 0) ? true : false;
}


void* queue_get_first(Queue* q) {
    assert(q != NULL);

    if (true == queue_is_empty(q)) {
        return NULL;
    }

    return q->list->node->data;
}


void queue_delete_first(Queue* q) {
    assert(q != NULL);

    if (true == queue_is_empty(q)) {
        return;
    }

    list_delete_node(q->list, q->list->node);
}


void* queue_insert(Queue* q, void* data) {
    assert(q != NULL && data != NULL);

    list_insert_data_last(q->list, data);

    return data;
}


void queue_destruct(Queue* q) {
    assert(q != NULL);

    list_destruct(q->list);
    free(q->list);
}


size_t queue_get_size(Queue const* q) {
    assert(q != NULL);

    return list_get_size(q->list);
}


/* ---------------------------------------------------------------------------------------------------- */
#ifndef NDEBUG

#include <stdio.h>
#include <string.h>

#define MAX_SIZE 10
static int test_array[MAX_SIZE] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
static char const* test_words[] = {"Apple", "Orange", "Banana", "Lemon", "Lime", "Strawberry"};
#define TEST_WORDS_SIZE (sizeof(test_words) / sizeof(test_words[0]))
static int const check_size = MAX_SIZE;


static void str_release_func(void* d) {
    free(*(char**)d);
    free(d);
}


int main(void) {
    Queue q;
    Queue* const qp = &q;
    queue_init(qp, sizeof(int), NULL, false);

    assert(queue_get_first(qp) == NULL);
    assert(queue_get_size(qp) == 0);

    printf("queue_insert -----------------------\n");
    for (int i = 0; i < check_size; i++) {
        queue_insert(qp, &test_array[i]);
        printf("%d ", test_array[check_size - i - 1]);
        assert(*(int*)queue_get_first(qp) == test_array[0]);
    }
    assert(queue_get_size(qp) == check_size);
    printf("\n-------------------------------\n");

    printf("queue_dequeue -----------------------\n");
    for (int i = 0; i < check_size; i++) {
        int n = *(int*)queue_get_first(qp);
        printf("%d ", n);
        assert(n == test_array[i]);
        queue_delete_first(qp);
    }
    assert(queue_get_size(qp) == 0);
    printf("\n-------------------------------\n");

    queue_destruct(qp);

    printf("Release func-------------------\n");
    queue_init(qp, sizeof(char*), str_release_func, true);
    for (int i = 0; i < TEST_WORDS_SIZE; i++) {
        char* c = (char*)malloc(strlen(test_words[i]));
        strcpy(c, test_words[i]);
        printf("queue_insert and queue_dequeue: %s\n", test_words[i]);
        queue_insert(qp, &c);
        assert(strcmp(*(char**)queue_get_first(qp), test_words[i]) == 0);
        queue_delete_first(qp);
    }
    assert(queue_get_size(qp) == 0);
    queue_destruct(qp);
    assert(queue_get_size(qp) == 0);
    printf("Relese All element\n");
    printf("-------------------------------\n");

    printf("All Test Passed.\n");

    return 0;
}


#endif
/* ---------------------------------------------------------------------------------------------------- */
