/**
 * @file buddy_system.c
 * @brief This is CUI simulater of Buddy System algorithm for x86_32.
 * @author mopp
 * @version 0.1
 * @date 2014-09-10
 */

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "minunit.h"


/* The number of frame  : 1 2 4 8 16 32 64 128 256 512 1024 */
/* Order in buddy system: 0 1 2 3  4  5  6   7   8   9   10 */
#define BUDDY_SYSTEM_MAX_ORDER 11

#define FRAME_SIZE 0x1000U /* frame size is 4 KB. */
// #define MAX_MEMORY_SIZE (0xffffffffU) /* max memory size of x86_32 is 4GB */
#define MAX_MEMORY_SIZE (1024 * 1024) /* max memory size of x86_32 is 4GB */
// 1MB


typedef struct elist {
    struct elist* next;
    struct elist* prev;
} Elist;


struct frame {
    Elist list;
    uint8_t status;
    uint8_t order;
};
typedef struct frame Frame;


enum frame_constants {
    FRAME_STATE_FREE = 0,
    FRAME_STATE_ALLOC,
};


struct buddy_manager {
    Frame* frame_pool;
    Elist frames[BUDDY_SYSTEM_MAX_ORDER]; /* This list is dummy. actually element is after list->next. */
    size_t free_page_nr[BUDDY_SYSTEM_MAX_ORDER];
    size_t total_frame_nr;
    size_t free_memory_size;
    size_t alloc_memory_size;
};
typedef struct buddy_manager Buddy_manager;


#define elist_get_element(type, list) (type)(list)

#define elist_foreach(type, var, list) \
for (type var = elist_get_element(type, (list)->next); (uintptr_t)var != (uintptr_t)(list); var = elist_get_element(type, ((Elist*)var)->next))

static inline Elist* elist_init(Elist* l) {
    l->next = l;
    l->prev = l;
    return l;
}


static inline Elist* elist_insert_next(Elist* l, Elist* new) {
    new->next = l->next;
    new->prev = l;
    l->next = new;
    new->next->prev = new;

    return l;
}


static inline Elist* elist_insert_prev(Elist* l, Elist* new) {
    return elist_insert_next(l->prev, new);
}


static inline Elist* elist_remove(Elist* n) {
    Elist* next = n->next;
    Elist* prev = n->prev;
    prev->next = next;
    next->prev = prev;

    return n;
}


static inline bool elist_is_empty(Elist* n) {
    return NULL == n;
}


static inline size_t get_frame_idx(Buddy_manager const* const bman, Frame const* const frame) {
    assert(bman != NULL);
    assert(frame != NULL);
    return ((uintptr_t)frame - (uintptr_t)bman->frame_pool) / sizeof(Frame);
}


static inline uintptr_t get_frame_addr(Buddy_manager const* const bman, Frame const* const frame) {
    assert(bman != NULL);
    assert(frame != NULL);
    return get_frame_idx(bman, frame) * FRAME_SIZE;
}


static inline uintptr_t get_frame_addr_by_idx(Buddy_manager const* const bman, size_t idx) {
    assert(bman != NULL);
    assert(idx <= bman->total_frame_nr);
    return get_frame_addr(bman, &bman->frame_pool[idx]);
}

/*
 * バディは2の累乗であることを利用する
 * xor は そのビットが1であれば0に、0であれば1にする
 * つまり、アドレスの小さいバディであれば足し、大きいバディであれば引くという処理になる.
 * オーダーが1の時、要素0番のバディは要素2番
 * 0 + (1 << 1) = 2
 */
static inline Frame* get_buddy_frame_by_idx(Buddy_manager const* const bman, size_t idx, uint8_t order) {
    return bman->frame_pool + (idx ^ (1 << order));
}


static inline Frame* get_buddy_frame(Buddy_manager const* const bman, Frame const * const frame, uint8_t order) {
    return get_buddy_frame_by_idx(bman, get_frame_idx(bman, frame), order);
}


static inline bool is_frame_your_buddy(Frame const * const you, Frame const * const buddy) {
    return you->order == buddy->order;
}


Buddy_manager* init_buddy_system(Buddy_manager* bman, uint32_t memory_size) {
    size_t frame_nr = memory_size / FRAME_SIZE;

    assert(bman != NULL);
    assert(memory_size != 0);
    assert(frame_nr != 0);

    printf("memory_size = %uKB\n", memory_size / 1024);
    printf("frame_nr    = %lu\n", frame_nr);

    Frame* frames = malloc(sizeof(Frame) * frame_nr);
    if (frames == NULL) {
        return NULL;
    }

    bman->frame_pool        = frames;
    bman->total_frame_nr    = frame_nr;
    bman->free_memory_size  = memory_size;
    bman->alloc_memory_size = 0;
    bman->free_page_nr[0]   = frame_nr;
    for (uint8_t i = 1; i < BUDDY_SYSTEM_MAX_ORDER; ++i) {
        bman->free_page_nr[i] = 0;
    }

    Frame* itr;
    Frame* const end = frames + frame_nr;

    for (uint8_t i = 0; i < BUDDY_SYSTEM_MAX_ORDER; ++i) {
        itr = frames;
        do {
            Frame* bf = get_buddy_frame(bman, itr, i);
            itr->order = bf->order = i;
            itr = bf;
        } while (++itr <= end);
    }

    itr = frames;
    do {
        printf("%02d ", itr->order);
    } while (++itr <= end);
    putchar('\n');

    // elist_foreach(Frame*, itr, bman->frames[0]) {
    //     printf("%02d ", itr->order);
    // }
    // putchar('\n');

    /* [0] [0] [0] [0] [0] [0] [0] [0] [0] [0]  */
    /* [1 1] [1 1] [1 1] [1 1] [1 1] [0]  */
    /* [2 2 2 2] [2 2 2 2] [0 0] [0]  */
    /* [3 3 3 3 3 3 3 3] [0 0] [0]  */

    return bman;

    bman->frames[0]   = frames[0].list;
    elist_init(&bman->frames[0]);
    while (++frames < end) {
        elist_insert_prev(&bman->frames[0], &frames->list);
    }

    printf("%lu\n", get_frame_addr(bman, bman->frame_pool + 1));

    return bman;
}


static char const* test_elist_foreach(void) {
    struct number {
        Elist list;
        int num;
    };

    size_t nr = 10;
    struct number n[nr];

    for (int i = 0; i < nr; i++) {
        n[i].num = i;
    }
    /* int nums[] = {0, 9, 8, 7, 6, 5, 4, 3, 2, 1}; */

    Elist* head = elist_init(&(n[0].list));
    MIN_UNIT_ASSERT("ERROR: elist_init is wrong.", NULL !=  head);

    for (size_t i = 1; i < nr; i++) {
        elist_insert_next(head, &(n[i].list));
        MIN_UNIT_ASSERT("ERROR: elist_insert_next is wrong.", head->next == (Elist*)&n[i]);
    }

    int cnt = 0;
    elist_foreach(struct number*, i, head) {
        i->num = cnt++;
    }

    cnt = 0;
    elist_foreach(struct number*, i, head) {
        MIN_UNIT_ASSERT("ERROR: elist_foreach is wrong.", i->num == cnt++);
        printf("%02d ", i->num);
    }
    putchar('\n');


    return NULL;
}


static char const* test_get_frame_addr(void) {
    Buddy_manager bman;
    bman.frame_pool = malloc(sizeof(Frame) * 10);

    MIN_UNIT_ASSERT("ERROR: get_frame_addr is wrong.", 0 == get_frame_addr(&bman, bman.frame_pool));
    MIN_UNIT_ASSERT("ERROR: get_frame_addr is wrong.", FRAME_SIZE == get_frame_addr(&bman, bman.frame_pool + 1));
    MIN_UNIT_ASSERT("ERROR: get_frame_addr is wrong.", FRAME_SIZE * 10 == get_frame_addr(&bman, bman.frame_pool + 10));

    free(bman.frame_pool);

    return NULL;
}


static char const* test_get_buddy_frame(void) {
    Buddy_manager bman;
    bman.frame_pool = malloc(sizeof(Frame) * 256);

    MIN_UNIT_ASSERT("ERROR: get_buddy_frame is wrong.", &bman.frame_pool[1] == get_buddy_frame(&bman, &bman.frame_pool[0], 0));
    MIN_UNIT_ASSERT("ERROR: get_buddy_frame is wrong.", &bman.frame_pool[0] == get_buddy_frame(&bman, &bman.frame_pool[1], 0));
    MIN_UNIT_ASSERT("ERROR: get_buddy_frame is wrong.", &bman.frame_pool[2] == get_buddy_frame(&bman, &bman.frame_pool[0], 1));
    MIN_UNIT_ASSERT("ERROR: get_buddy_frame is wrong.", &bman.frame_pool[8] == get_buddy_frame(&bman, &bman.frame_pool[0], 3));

    free(bman.frame_pool);

    return NULL;
}


static char const* all_tests(void) {
    MIN_UNIT_RUN(test_elist_foreach);
    MIN_UNIT_RUN(test_get_frame_addr);
    MIN_UNIT_RUN(test_get_buddy_frame);
    return NULL;
}


int do_all_tests(void) {
    MIN_UNIT_RUN_ALL(all_tests);
}


int main(void) {
    do_all_tests();
    printf("============================================================\n");
    printf("size of Buddy_manager = %lu Byte\n", sizeof(Buddy_manager));

    Buddy_manager bman;

    if (init_buddy_system(&bman, FRAME_SIZE * 10) == NULL) {
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
