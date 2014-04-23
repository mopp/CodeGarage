/**
 * @file doubly_circularly_linked_list.c
 * @brief DoublyCircularlyLinkedList.
 *      tail                            head
 *          first->X0->X1->last->X0->...
 *          last<-X1<-X0<-last<-X1<-...
 * @author mopp
 * @version 0.1
 * @date 2014-04-23
 */
/* #define NDBUG */

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <assert.h>

#include <string.h>
#include <stdio.h>
#include <stdlib.h>


/* free function for list node. */
typedef void (*free_func)(void*);
/* comparison function for list node. */
typedef bool (*comp_func)(void*, void*);
/*
 * for each function for list node.
 * if return value is false, loop is abort
 */
typedef bool (*loop_func)(void*);


struct dlinked_list_node {
    void* data;
    struct dlinked_list_node* head;
    struct dlinked_list_node* tail;
};
typedef struct dlinked_list_node Dlinked_list_node;


struct dlinked_list {
    Dlinked_list_node* first;
    Dlinked_list_node* last;
    free_func free;
    size_t size;
    size_t data_type_size;
};
typedef struct dlinked_list Dlinked_list;


/* inisialize list */
Dlinked_list* init_list(Dlinked_list* l, size_t size, free_func f) {
    l->first = NULL;
    l->last = NULL;
    l->free = f;
    l->size = 0;
    l->data_type_size = size;

    return l;
}


/* allocate new node and set data in it. */
Dlinked_list_node* get_new_dlinked_list_node(Dlinked_list* l, void* data) {
    Dlinked_list_node* n = (Dlinked_list_node*)malloc(sizeof(Dlinked_list_node));
    if (n == NULL) {
        return NULL;
    }

    /* copy data to new area */
    n->data = malloc(l->data_type_size);
    if (n->data == NULL) {
        return NULL;
    }
    memcpy(n->data, data, l->data_type_size);

    n->head = NULL;
    n->tail = NULL;

    return n;
}


/* all node in list will be freed. */
void destruct_list(Dlinked_list* l) {
    if (l->first == NULL && l->last == NULL) {
        return;
    }

    /* select default free function or user free function. */
    free_func f = (l->free != NULL) ? l->free : free;

    if (l->first == l->last) {
        f(l->first->data);
        free(l->first);
        l->first = l->last = NULL;
    }

    Dlinked_list_node* n = l->first;
    Dlinked_list_node* t;
    do {
        t = n->tail;
        f(n->data);
        free(n);
        n = t;
    } while (n != l->last);

    f(t->data);
    free(t);

    l->last = l->first = NULL;
    l->size = 0;
}


/*
 * add new node into head of argument node.
 * @return pointer to new node.
 */
Dlinked_list_node* insert_head(Dlinked_list* l, Dlinked_list_node* target, void* data) {
    assert(l != NULL && target != NULL);

    Dlinked_list_node* new = get_new_dlinked_list_node(l, data);

    new->head = target->head;
    if (new->head != NULL) {
        new->head->tail = new;
    }

    new->tail = target;
    if (new->tail != NULL) {
        new->tail->head = new;
    }

    if (target == l->first) {
        l->first = new;
    }

    ++(l->size);

    return new;
}


/*
 * add new node into tail of argument node.
 * @return pointer to new node.
 */
Dlinked_list_node* insert_tail(Dlinked_list* l, Dlinked_list_node* target, void* data) {
    assert(l != NULL && target != NULL);

    Dlinked_list_node* new = get_new_dlinked_list_node(l, data);

    new->tail = target->tail;
    if (new->tail != NULL) {
        new->tail->head = new;
    }

    new->head = target;
    if (new->head != NULL) {
        target->tail = new;
    }

    if (target == l->last) {
        l->last = new;
    }

    ++(l->size);

    return new;
}


static inline void insert_first_node(Dlinked_list* l, void* data) {
    assert(l->first == NULL && l->last == NULL);

    l->last = l->first = get_new_dlinked_list_node(l, data);
    l->first->head = l->first;
    l->first->tail = l->first;

    ++l->size;

    assert(l->first != NULL && l->last != NULL);
}


/*
 * add new node before first node in list
 * @return pointer to list.
 */
Dlinked_list* insert_first(Dlinked_list* l, void* data) {
    if (l->first == NULL && l->last == NULL) {
        insert_first_node(l, data);
    } else {
        insert_head(l, l->first, data);
    }

    return l;
}


/*
 * add new node after last node in list
 * @return pointer to list.
 */
Dlinked_list* insert_last(Dlinked_list* l, void* data) {
    if (l->first == NULL && l->last == NULL) {
        insert_first_node(l, data);
    } else {
        l->last = insert_tail(l, l->last, data);
    }

    return l;
}


/* delete node in list. */
void delete_node(Dlinked_list* l, Dlinked_list_node* target) {
    target->head->tail = target->tail;
    target->tail->head = target->head;

    if (l->free != NULL) {
        l->free(target);
    } else {
        free(target);
    }

    --l->size;
}


/* search node witch has argument data. */
Dlinked_list_node* search_node(Dlinked_list* l, void* data, comp_func f) {
    if (l->first == NULL || l->last == NULL) {
        return NULL;
    }

    for (Dlinked_list_node* n = l->first; n != l->last; n = n->tail) {
        if (f(n->data, data) == true) {
            return n;
        }
    }

    return NULL;
}


Dlinked_list* list_for_each(Dlinked_list* const l, loop_func const f, bool const is_reverse) {
    assert(f != NULL);

    if (l->first == NULL || l->last == NULL) {
        assert(0);
        return l;
    }

    Dlinked_list_node* n;
    if (is_reverse == true) {
        /* last to first loop */
        n = l->last;
        do {
            if (false == f(n->data)) {
                break;
            }
            n = n->head;
        } while (n != l->first);
    } else {
        /* first to last loop */
        n = l->last;
        n = l->first;
        do {
            if (false == f(n->data)) {
                break;
            }
            n = n->tail;
        } while (n != l->last);
    }
    f(n->data);

    return l;
}


/* ---------------------------------------------------------------------------------------------------- */


static void print_list(Dlinked_list const* const l) {
    printf("-------------------------------\n");
    printf("first          : %p\n", l->first);
    printf("last           : %p\n", l->last);
    printf("data_type_size : %ld\n", l->data_type_size);
    printf("size           : %ld\n", l->size);
    printf("-------------------------------\n");
}


bool loop_int(void* d) {
    printf("%d -> ", *(int*)d);

    return true;
}


int main(void) {
    int const loop_cnt = 10;
    Dlinked_list l;
    init_list(&l, sizeof(int), NULL);

    print_list(&l);

    for (int i = 0; i < loop_cnt; i++) {
        insert_first(&l, &i);
    }

    print_list(&l);

    printf("first -> ");
    list_for_each(&l, loop_int, false);
    puts("last");

    printf("last -> ");
    list_for_each(&l, loop_int, true);
    puts("first");


    printf("Destruct\n");
    destruct_list(&l);
    print_list(&l);

    return 0;
}


/* ---------------------------------------------------------------------------------------------------- */
