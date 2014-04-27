/*
 * @file doubly_circularly_linked_list.c
 * @brief This list is DoublyCircularlyLinkedList.
 *      prev                            next
 *          node(X0)<->X1<->node(X0)->X1->...
 * @author mopp
 * @version 0.2
 * @date 2014-04-25
 */
/* #define NDBUG */

#include <assert.h>
#include <string.h>
#include <stdlib.h>

#include "doubly_circularly_linked_list.h"


/**
 * @brief initialize list.
 * @param l pointer to list.
 * @param size size of stored data type in list.
 * @param f pointer to function for release data in list.
 * @return pointer to list.
 */
List* list_init(List* l, size_t size, release_func f) {
    l->node = NULL;
    l->free = f;
    l->size = 0;
    l->data_type_size = size;

    return l;
}


/**
 * @brief allocate new node and set data in it.
 * @param l pointer to list.
 * @param data pointer to set data into new node.
 * @return pointer to new node.
 */
List_node* list_get_new_node(List* l, void* data) {
    List_node* n = (List_node*)malloc(sizeof(List_node));
    if (n == NULL) {
        return NULL;
    }

    n->next = n->prev = NULL;

    /* copy data to new area */
    n->data = malloc(l->data_type_size);
    if (n->data == NULL) {
        return NULL;
    }
    memcpy(n->data, data, l->data_type_size);

    return n;
}


/**
 * @brief add new node into next of second argument node.
 *        after execute, list became "... -> target -> new -> ...".
 * @param l pointer to list.
 * @param target pointer to base list.
 * @param new pointer to added node.
 * @return pointer to added node(third argument).
 */
List_node* list_insert_node_next(List* l, List_node* target, List_node* new) {
    assert(l != NULL && target != NULL);

    new->next = target->next;
    new->next->prev = new;

    new->prev = target;
    new->prev->next = new;

    ++(l->size);

    return new;
}


/**
 * @brief add new data into next of argument node.
 *        And allocate new node for data of argument.
 *        after execute, list is "... -> target -> new(has data) -> ...".
 * @param l pointer to list.
 * @param target pointer to base list.
 * @param data pointer to added data.
 * @return pointer to allocated node.
 */
List_node* list_insert_data_next(List* l, List_node* target, void* data) {
    assert(l != NULL && target != NULL);

    return list_insert_node_next(l, target, list_get_new_node(l, data));
}


/**
 * @brief add new node into previous of argument node.
 *        after execute, list is "... -> new -> target -> ...".
 * @param l pointer to list.
 * @param target pointer to base node.
 * @param new pointer to added node.
 * @return pointer to added node(third argument).
 */
List_node* list_insert_node_prev(List* l, List_node* target, List_node* new) {
    assert(l != NULL && target != NULL);

    new->prev = target->prev;
    new->prev->next = new;

    new->next = target;
    target->prev = new;

    ++(l->size);

    return new;
}


/**
 * @brief add new data into prev of argument node.
 *        And allocate new node for data of argument.
 *        after execute, list is "... -> new(has data) -> target -> ...".
 * @param l pointer to list.
 * @param target pointer to base node.
 * @param data pointer to set data into new node.
 * @return pointer to allocated node.
 */
List_node* list_insert_data_prev(List* l, List_node* target, void* data) {
    assert(l != NULL && target != NULL);

    return list_insert_node_prev(l, target, list_get_new_node(l, data));
}


/**
 * @brief insert node when list has NOT any node.
 * @param l pointer to list.
 * @param new pointer to added node.
 */
static inline void list_insert_first_node(List* l, List_node* new) {
    assert(l->node == NULL);

    /* set pointer to self. */
    l->node = new;
    l->node->next = l->node->prev = l->node;

    ++l->size;

    assert(l->node != NULL);
}


/**
 * @brief add new node at first in argument list.
 * @param l pointer to list.
 * @param new pointer to added node.
 * @return pointer to list.
 */
List* list_insert_node_first(List* l, List_node* new) {
    if (l->node == NULL) {
        list_insert_first_node(l, new);
    } else {
        l->node = list_insert_node_prev(l, l->node, new);
    }

    return l;
}


/**
 * @brief add new data into first position in argument list.
 * @param l pointer to list.
 * @param data pointer to added data.
 * @return pointer to list.
 */
List* list_insert_data_first(List* l, void* data) {
    return list_insert_node_first(l, list_get_new_node(l, data));
}



/**
 * @brief add new node into last positio in argument list.
 * @param l pointer to list.
 * @param new pointer to added node.
 * @return pointer to list.
 */
List* list_insert_node_last(List* l, List_node* new) {
    if (l->node == NULL) {
        list_insert_first_node(l, new);
    } else {
        l->node->prev = list_insert_node_next(l, l->node->prev, new);
    }

    return l;
}


/**
 * @brief add new node after last node in list
 * @param l pointer to list.
 * @param data pointer to adde data.
 * @return pointer to list.
 */
List* list_insert_data_last(List* l, void* data) {
    return list_insert_node_last(l, list_get_new_node(l, data));
}



/**
 * @brief remove argument node in list.
 *        And this NOT releases data.
 *        therefor, You MUST release data yourself.
 * @param l pointer to list.
 * @param target pointer to deleted node.
 * @return pointer to removed node.
 */
List_node* list_remove_node(List* l, List_node* target) {
    assert(target != NULL);

    if (l->size == 1) {
        /*
         * size equals 1.
         * this means that list has only "node".
         */
        l->node = NULL;
    } else if (target == l->node) {
        /* change "node". */
        l->node->prev->next = target->next;
        target->next->prev = l->node->prev;
        l->node = target->next;
    } else {
        target->next->prev = target->prev;
        target->prev->next = target->next;
    }

    --l->size;

    return target;
}


/**
 * @brief delete argument node in list.
 *        And this releases data.
 * @param l pointer to list.
 * @param target pointer to deleted node.
 */
void list_delete_node(List* l, List_node* target) {
    assert(target != NULL);

    ((l->free == NULL) ? (free) : (l->free))(list_remove_node(l, target));
}


/**
 * @brief all node in list be freed.
 * @param l pointer to list.
 */
void list_destruct(List* l) {
    if (l->node == NULL) {
        /* do nothing. */
        return;
    }

    /* select default free function or user free function. */
    release_func f = (l->free != NULL) ? l->free : free;

    List_node* n = l->node;
    List_node* t = l->node;
    List_node* limit = l->node->prev;

    if (l->size != 1) {
        do {
            t = n->next;
            f(n->data);
            free(n);
            n = t;
        } while (n != limit);
    }
    f(t->data);
    free(t);

    l->node = NULL;
    l->size = 0;
}


/**
 * @brief get the number of node in list.
 * @param l pointer to list.
 * @return the number of node.
 */
size_t list_get_size(List const* l) {
    return l->size;
}


/**
 * @brief this function provide for_each loop based on argument node.
 * @param l pointer to list.
 * @param f pointer to function witch decide stop or continue loop.
 * @param is_reverse loop direction flag. if it is true, loop is first to last.
 * @return pointer to node when loop stoped or NULL.
 */
List_node* list_node_for_each(List* const l, List_node* n, for_each_func const f, bool const is_reverse) {
    assert(f != NULL);

    if (l->node == NULL) {
        return NULL;
    }

    if (l->node == l->node->next) {
        /* when list has only one node. */
        f(l->node->data);
        return l->node;
    }

    /* List_node* n = (true == is_reverse) ? l->node->prev : l->node->next; */
    List_node const* const stored = n;
    do {
        if (true == f(n->data)) {
            return n;
        }
        n = (true == is_reverse) ? n->prev : n->next;
    } while (n != stored);

    return NULL;
}


/**
 * @brief this function provide for_each loop.
 * @param l pointer to list.
 * @param f pointer to function witch decide stop or continue loop.
 * @param is_reverse loop direction flag. if it is true, loop is first to last.
 * @return pointer to node when loop stoped or NULL.
 */
List_node* list_for_each(List* const l, for_each_func const f, bool const is_reverse) {
    assert(f != NULL);
    return list_node_for_each(l, ((true == is_reverse) ? l->node->prev : l->node), f, is_reverse);
}


static void* search_target;
static List* search_list;
static bool search_loop(void* data) {
    return (0 == memcmp(search_target, data, search_list->data_type_size)) ? true : false;
}


/**
 * @brief search node witch has argument data.
 * @param l pointer to list.
 * @param data pointer to search key data.
 * @return
 */
List_node* list_search_node(List* l, void* data) {
    if (l->node == NULL) {
        return NULL;
    }

    search_target = data;
    search_list = l;

    List_node* n = list_for_each(l, search_loop, false);

    search_target = NULL;
    search_list = NULL;

    return (n == NULL) ? NULL : n;
}



/* ---------------------------------------------------------------------------------------------------- */
#ifndef NDEBUG


#include <stdio.h>

#define MAX_SIZE 10
static int test_array[MAX_SIZE] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
static char const* test_words[] = {"Apple", "Orange", "Banana", "Lemon", "Lime", "Strawberry"};
#define TEST_WORDS_SIZE (sizeof(test_words) / sizeof(test_words[0]))
static int const check_size = MAX_SIZE;

static void print_list(List const* const l) {
    printf("Print List---------------------\n");
    printf("node           : %p\n", l->node);
    printf("data in node   : %d\n", (l->node != NULL) ? *(int*)l->node->data : -1);
    printf("data_type_size : %ld\n", l->data_type_size);
    printf("size           : %ld\n", list_get_size(l));
    printf("-------------------------------\n");
}


static void print_node(List_node const* const n) {
    if (n == NULL) {
        return;
    }
    printf("Print Node---------------------\n");
    printf("data        : %d\n", *(int*)n->data);
    printf("next        : %p\n", n->next);
    printf("prev        : %p\n", n->prev);
    printf("-------------------------------\n");
}


static inline bool echo(void* d) {
    printf("%d -> ", *(int*)d);
    return false;
}


static bool test_loop_int_rev(void* d) {
    static int cnt = 0;

    echo(d);
    assert(*(int*)d == test_array[cnt++ % check_size]);

    return false;
}


static bool test_loop_int(void* d) {
    static int cnt = 0;

    echo(d);
    assert(*(int*)d == test_array[check_size - cnt++ % check_size - 1]);

    return false;
}


static bool test_loop_str(void* d) {
    printf("%s -> ", *(char**)d);
    return false;
}


static void test_destract(List* l) {
    printf("Destruct List------------------\n");
    list_destruct(l);
    assert(list_get_size(l) == 0 && l->node == NULL);
    printf("Delete all node in List\n");
    printf("-------------------------------\n");
}


static void str_release_func(void* d) {
    static int cnt = 0;
    assert(strcmp(test_words[cnt++], *(char**)d) == 0);

    free(*(char**)d);
    free(d);
}


int main(void) {
    List l;
    list_init(&l, sizeof(int), NULL);

    print_list(&l);
    print_node(l.node);

    printf("Insert First-------------------\n");
    for (int i = 0; i < check_size; i++) {
        list_insert_data_first(&l, test_array + i);
    }
    assert(list_get_size(&l) == check_size);

    printf("first -> ");
    list_for_each(&l, test_loop_int, false);
    puts("last");

    printf("last -> ");
    list_for_each(&l, test_loop_int_rev, true);
    puts("first");
    printf("-------------------------------\n");

    test_destract(&l);

    print_list(&l);

    printf("Insert Last-------------------\n");
    for (int i = 0; i < check_size; i++) {
        list_insert_data_last(&l, test_array + i);
    }

    printf("first -> ");
    list_for_each(&l, test_loop_int_rev, false);
    puts("last");

    printf("last -> ");
    list_for_each(&l, test_loop_int, true);
    puts("first");
    printf("-------------------------------\n");

    printf("Search node--------------------\n");
    for (int i = 0; i < check_size; i++) {
        int ti = i + 1;
        List_node* t = list_search_node(&l, &ti);
        if (t != NULL) {
            printf("%2d found\n", *(int*)t->data);
        } else {
            printf("%2d NOT found\n", ti);
        }
        assert(*(int*)t->data == ti);
    }
    printf("-------------------------------\n");

    test_destract(&l);

    printf("Insert first/next/prev---------\n");
    int a = 10, b = 20, c = 50;
    list_insert_data_first(&l, &a);
    assert(*(int*)(l.node->data) == a);

    list_insert_data_next(&l, l.node, &b);
    assert(*(int*)l.node->next->data == b);

    list_insert_data_prev(&l, l.node, &c);
    assert(*(int*)l.node->prev->data == c);

    printf("first -> ");
    list_for_each(&l, echo, false);
    puts("last");
    printf("-------------------------------\n");

    printf("Delete Node--------------------\n");
    list_delete_node(&l, l.node);
    assert(*(int*)l.node->data == b);

    list_delete_node(&l, l.node);
    assert(*(int*)l.node->data == c);

    list_delete_node(&l, l.node);
    assert(list_get_size(&l) == 0 && l.node == NULL);

    print_list(&l);
    printf("-------------------------------\n");

    test_destract(&l);

    printf("Release func-------------------\n");
    list_init(&l, sizeof(char*), str_release_func);
    for (int i = 0; i < TEST_WORDS_SIZE; i++) {
        char* c = (char*)malloc(strlen(test_words[i]));
        strcpy(c, test_words[i]);
        list_insert_data_last(&l, &c);
    }

    printf("first -> ");
    list_for_each(&l, test_loop_str, false);
    puts("last");
    printf("first -> ");
    list_node_for_each(&l, l.node->next->next, test_loop_str, false);
    puts("last");
    assert(list_get_size(&l) == TEST_WORDS_SIZE);

    printf("-------------------------------\n");

    test_destract(&l);

    printf("\nAll Test Passed !\n");

    return 0;
}


#endif
/* ---------------------------------------------------------------------------------------------------- */
