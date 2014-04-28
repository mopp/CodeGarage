/**
 * @file array_queue.c
 * @brief queue by array.
 * @author mopp
 * @version 0.1
 * @date 2014-04-27
 */
#define NDEBUG

#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "array_queue.h"


Aqueue* aqueue_init(Aqueue* q, size_t t_size, size_t capacity, release_func f) {
    assert(q != NULL);

    q->data = (void**)malloc(sizeof(void*) * capacity);
    q->first = q->last = 0;
    q->capacity = capacity;
    q->size = 0;
    q->data_type_size = t_size;
    q->free = f;

    return q;
}


bool aqueue_is_empty(Aqueue const* q) {
    assert(q != NULL);

    return (aqueue_get_size(q) == 0) ? true : false;
}


bool aqueue_is_full(Aqueue const* q) {
    assert(q != NULL);

    return (aqueue_get_size(q) == q->capacity) ? true : false;
}


void* aqueue_get_first(Aqueue* q) {
    assert(q != NULL);

    if (true == aqueue_is_empty(q)) {
        return NULL;
    }

    return q->data[q->first];
}


void aqueue_delete_first(Aqueue* q) {
    assert(q != NULL);

    if (aqueue_is_empty(q) == true || q->data[q->first] == NULL) {
        return;
    }

    ((q->free != NULL) ? q->free : free)(q->data[q->first]);
    q->data[q->first] = NULL;

    q->first = (q->first + 1 == q->capacity) ? 0 : q->first + 1;

    --q->size;
}


void* aqueue_insert(Aqueue* q, void* data) {
    if (aqueue_is_full(q) == true) {
        return NULL;
    }

    q->data[q->last] = malloc(q->data_type_size);

    memcpy(q->data[q->last], data, q->data_type_size);

    q->last = (q->last + 1 == q->capacity) ? 0 : q->last + 1;

    ++q->size;

    return data;
}


void aqueue_destruct(Aqueue* q) {
    assert(q != NULL);

    size_t const size = aqueue_get_size(q);
    for (int i = 0; i < size; i++) {
        aqueue_delete_first(q);
    }

    free(q->data);
}


size_t aqueue_get_size(Aqueue const* q) {
    assert(q != NULL);

    return q->size;
}


size_t aqueue_get_capacity(Aqueue const* q) {
    assert(q != NULL);

    return q->capacity;
}



/* ---------------------------------------------------------------------------------------------------- */
#ifndef NDEBUG

#include <stdio.h>

#define MAX_CAPACITY 10
static char const* test_words[] = {"Apple", "Orange", "Banana", "Lemon", "Lime", "Strawberry"};
#define TEST_WORDS_SIZE (sizeof(test_words) / sizeof(test_words[0]))


static void str_release_func(void* d) {
    free(*(char**)d);
    free(d);
}


int main(void) {
    Aqueue aq;
    Aqueue* const p = &aq;

    printf("Array Init --------------------\n");
    aqueue_init(p, sizeof(int), MAX_CAPACITY, NULL);
    assert(aqueue_is_empty(p) == true);
    assert(aqueue_get_capacity(p) == MAX_CAPACITY);
    assert(aqueue_get_size(p) == 0);
    printf("-------------------------------\n");

    printf("Array Insert ------------------\n");
    for (int i = 0; i < MAX_CAPACITY; i++) {
        aqueue_insert(p, &i);
        assert(*(int*)aqueue_get_first(p) == 0);
    }
    assert(aqueue_get_size(p) == MAX_CAPACITY);
    for (int i = 0; i < MAX_CAPACITY; i++) {
        int* t = (int*)aqueue_get_first(p);
        printf("out %d\n", *t);
        assert(*t == i);
        aqueue_delete_first(p);
    }
    assert(aqueue_is_empty(p) == true);
    aqueue_destruct(p);
    printf("-------------------------------\n");

    printf("Array func --------------------\n");
    aqueue_init(p, sizeof(char*), MAX_CAPACITY, str_release_func);
    for (int i = 0; i < TEST_WORDS_SIZE; i++) {
        char* c = (char*)malloc(sizeof(char) * strlen(test_words[i]));
        strcpy(c, test_words[i]);
        printf("%s\n", c);
        aqueue_insert(p, &c);
        assert(0 == strcmp(test_words[0], *(char**)aqueue_get_first(p)));
    }
    assert(aqueue_get_size(p) == TEST_WORDS_SIZE);
    assert(aqueue_is_empty(p) == false);
    printf("-------------------------------\n");

    printf("Array Destruct ----------------\n");
    aqueue_destruct(p);
    printf("-------------------------------\n");

    printf("All Test Passed.\n");

    return 0;
}

#endif
/* ---------------------------------------------------------------------------------------------------- */
