/**
 * @file lqueue.c
 * @brief Queue implemented by list.
 * @author mopp
 * @version 0.1
 * @date 2014-04-24
 */

#include <stdlib.h>
#include <assert.h>
#include "lqueue.h"


Lqueue* lqueue_init(Lqueue* q, size_t size, release_func f) {
    assert(q != NULL);

    q->list = (Dlist*)malloc(sizeof(Dlist));

    dlist_init(q->list, size, f);

    return q;
}


bool lqueue_is_empty(Lqueue const* q) {
    assert(q != NULL);

    return (dlist_get_size(q->list) == 0) ? true : false;
}


void* lqueue_get_first(Lqueue* q) {
    assert(q != NULL);

    if (true == lqueue_is_empty(q)) {
        return NULL;
    }

    return q->list->node->data;
}


void lqueue_delete_first(Lqueue* q) {
    assert(q != NULL);

    if (true == lqueue_is_empty(q)) {
        return;
    }

    dlist_delete_node(q->list, q->list->node);
}


void* lqueue_insert(Lqueue* q, void* data) {
    assert(q != NULL && data != NULL);

    dlist_insert_data_last(q->list, data);

    return data;
}


void lqueue_destruct(Lqueue* q) {
    assert(q != NULL);

    dlist_destruct(q->list);
    free(q->list);
}


size_t lqueue_get_size(Lqueue const* q) {
    assert(q != NULL);

    return dlist_get_size(q->list);
}
