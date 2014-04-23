/*
 * @file doubly_circularly_linked_list.c
 * @brief This list is DoublyCircularlyLinkedList.
 *      prev                            next
 *          node(X0)<->X1<->node(X0)->X1->...
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


/*
 * free function for list node.
 * It is used in destruct_list().
 */
typedef void (*release_func)(void*);
/*
 * comparison function for list node.
 * It is used in search_node().
 */
typedef bool (*comp_func)(void*, void*);
/*
 * for each function for list node.
 * if return value is true, loop is abort
 */
typedef bool (*for_each_func)(void*);


/*
 * List node structure.
 * It is in List structure below.
 */
struct list_node {
    void* data;             /* pointer to stored data in node. */
    struct list_node* next; /* pointer to next position node. */
    struct list_node* prev; /* pointer to previous position node. */
};
typedef struct list_node List_node;


/* List structure */
struct list {
    List_node* node;       /* start position pointer to node.
                            * and XXX: this node is first, this node->prev is last.
                            */
    release_func free;     /* function for releasing allocated data. */
    size_t size;           /* the number of node. */
    size_t data_type_size; /* it provided by sizeof(data). */
};
typedef struct list List;


/* inisialize list */
List* init_list(List* l, size_t size, release_func f) {
    l->node = NULL;
    l->free = f;
    l->size = 0;
    l->data_type_size = size;

    return l;
}


/* allocate new node and set data in it. */
List_node* get_new_list_node(List* l, void* data) {
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


/*
 * add new node into next of argument node.
 * after execute, list is "... -> target -> new_node -> ...".
 * @return pointer to new node.
 */
List_node* insert_next(List* l, List_node* target, void* data) {
    assert(l != NULL && target != NULL);

    List_node* new = get_new_list_node(l, data);

    new->next = target->next;
    new->next->prev = new;

    new->prev = target;
    new->prev->next = new;

    ++(l->size);

    return new;
}


/*
 * add new node into prev of argument node.
 * after execute, list is "... -> new_node -> target -> ...".
 * @return pointer to new node.
 */
List_node* insert_prev(List* l, List_node* target, void* data) {
    assert(l != NULL && target != NULL);

    List_node* new = get_new_list_node(l, data);

    new->prev = target->prev;
    new->prev->next = new;

    new->next = target;
    target->prev = new;

    ++(l->size);

    return new;
}


/* insert node when list has NOT any node. */
static inline void insert_first_node(List* l, void* data) {
    assert(l->node == NULL);

    /* set pointer to self. */
    l->node = get_new_list_node(l, data);
    l->node->next = l->node->prev = l->node;

    ++l->size;

    assert(l->node != NULL);
}


/*
 * add new node before first node in list
 * @return pointer to list.
 */
List* insert_first(List* l, void* data) {
    if (l->node == NULL) {
        insert_first_node(l, data);
    } else {
        l->node = insert_prev(l, l->node, data);
    }

    return l;
}


/*
 * add new node after last node in list
 * @return pointer to list.
 */
List* insert_last(List* l, void* data) {
    if (l->node == NULL) {
        insert_first_node(l, data);
    } else {
        l->node->prev = insert_next(l, l->node->prev, data);
    }

    return l;
}


/* delete argument node in list. */
void delete_node(List* l, List_node* target) {
    assert(target != NULL);

    if (l->size == 1) {
        l->node = NULL;
    } else if (target == l->node) {
        l->node->prev->next = target->next;
        target->next->prev = l->node->prev;
        l->node = target->next;
    } else {
        target->next->prev = target->prev;
        target->prev->next = target->next;
    }

    if (l->free != NULL) {
        l->free(target);
    } else {
        free(target);
    }

    --l->size;
}


/* all node in list will be freed. */
void destruct_list(List* l) {
    if (l->node == NULL) {
        /* do nothing. */
        return;
    }

    /* select default free function or user free function. */
    release_func f = (l->free != NULL) ? l->free : free;

    List_node* n = l->node;
    List_node* t = l->node;

    if (l->size != 1) {
        do {
            t = n->next;
            f(n->data);
            free(n);
            n = t;
        } while (n != l->node->prev);
    }
    f(t->data);
    free(t);

    l->node = NULL;
    l->size = 0;
}


/* for each loop. */
List_node* list_for_each(List* const l, for_each_func const f, bool const is_reverse) {
    assert(f != NULL);

    if (l->node == NULL) {
        assert(0);
        return NULL;
    }

    if (l->node == l->node->next) {
        /* when list has only one node. */
        f(l->node->data);
        return l->node;
    }

    if (is_reverse == false) {
        if (true == f(l->node->data)) {
            return l->node;
        }
    }

    List_node* n = (true == is_reverse) ? l->node->prev : l->node->next;
    do {
        if (true == f(n->data)) {
            break;
        }
        n = (true == is_reverse) ? n->prev : n->next;
    } while (n != l->node);

    if (true == is_reverse) {
        f(l->node->data);
    }

    return l->node;
}

static void* search_target;
static List* search_list;
static bool search_loop(void* data) {
    return (0 == memcmp(search_target, data, search_list->data_type_size));
}


/* search node witch has argument data. */
List_node* search_node(List* l, void* data) {
    if (l->node == NULL) {
        return NULL;
    }

    search_target = data;
    search_list = l;

    List_node* n = list_for_each(l, search_loop, false);

    search_target = NULL;
    search_list = NULL;

    return (n != NULL) ? n : NULL;
}



/* ---------------------------------------------------------------------------------------------------- */

#define MAX_SIZE 10
static int test_array[MAX_SIZE] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
static int const check_size = MAX_SIZE;

static void print_list(List const* const l) {
    printf("Print List---------------------\n");
    printf("node           : %p\n", l->node);
    printf("data in node   : %d\n", (l->node != NULL) ? *(int*)l->node->data : -1);
    printf("data_type_size : %ld\n", l->data_type_size);
    printf("size           : %ld\n", l->size);
    printf("-------------------------------\n");
}


static void print_node(List_node const* const n) {
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


static void test_destract(List* l) {
    printf("Destruct List------------------\n");
    destruct_list(l);
    assert(l->size == 0 && l->node == NULL);
    printf("Delete all node in List\n");
    printf("-------------------------------\n");
}


int main(void) {
    List l;
    init_list(&l, sizeof(int), NULL);

    print_list(&l);

    printf("Insert First-------------------\n");
    for (int i = 0; i < check_size; i++) {
        insert_first(&l, test_array + i);
    }

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
        insert_last(&l, test_array + i);
    }

    printf("first -> ");
    list_for_each(&l, test_loop_int_rev, false);
    puts("last");

    printf("last -> ");
    list_for_each(&l, test_loop_int, true);
    puts("first");
    printf("-------------------------------\n");

    test_destract(&l);

    printf("Insert first/next/prev---------\n");
    int a = 10, b = 20, c = 50;
    insert_first(&l, &a);
    print_node(l.node);
    assert(*(int*)(l.node->data) == a);
    insert_next(&l, l.node, &b);
    print_node(l.node->next);
    assert(*(int*)l.node->next->data == b);
    insert_prev(&l, l.node, &c);
    print_node(l.node->prev);
    assert(*(int*)l.node->prev->data == c);

    printf("first -> ");
    list_for_each(&l, echo, false);
    puts("last");
    printf("-------------------------------\n");

    printf("Delete Node--------------------\n");
    delete_node(&l, l.node);
    assert(*(int*)l.node->data == b);
    delete_node(&l, l.node);
    assert(*(int*)l.node->data == c);
    delete_node(&l, l.node);
    assert(l.size == 0 && l.node == NULL);
    print_list(&l);
    printf("-------------------------------\n");

    printf("\nAll Test Passed !\n");

    return 0;
}


/* ---------------------------------------------------------------------------------------------------- */
