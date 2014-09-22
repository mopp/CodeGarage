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
#define BUDDY_SYSTEM_MAX_ORDER (10 + 1)
#define BUDDY_SYSTEM_ORDER_NR(order) (1U << (order))

#define FRAME_SIZE 0x1000U /* frame size is 4 KB. */

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
    size_t free_frame_nr[BUDDY_SYSTEM_MAX_ORDER];
    size_t total_frame_nr;
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

    return elist_init(n);
}


static inline bool elist_is_empty(Elist* n) {
    return (n->next == n->prev) && (n == n->next);
}


static inline Frame* elist_get_frame(Elist const * const l) {
    return (Frame*)l;
}


static inline size_t get_frame_idx(Buddy_manager const* const bman, Frame const* const frame) {
    assert(bman != NULL);
    assert(frame != NULL);
    assert((uintptr_t)bman->frame_pool <= (uintptr_t)frame);
    return ((uintptr_t)frame - (uintptr_t)bman->frame_pool) / sizeof(Frame);
}


static inline uintptr_t get_frame_addr(Buddy_manager const* const bman, Frame const* const frame) {
    assert(bman != NULL);
    assert(frame != NULL);
    return get_frame_idx(bman, frame) * FRAME_SIZE;
}


/*
 * バディのアドレスが2の累乗であることを利用する.
 * xor は そのビットが1であれば0に、0であれば1にする.
 * ---------------------
 * | Buddy A | Buddy B |
 * ---------------------
 * 上図において、Buddy Aなら足して、Buddy Bを求める.
 * Buddy Bなら引いて、Buddy Aを求めるという処理になる.
 * オーダーが1の時、要素0番のバディは要素2番である.
 * 0 + (1 << 1) = 2
 */
static inline Frame* get_buddy_frame_by_idx(Buddy_manager const* const bman, size_t idx, uint8_t order) {
    return bman->frame_pool + (idx ^ BUDDY_SYSTEM_ORDER_NR(order));
}


static inline Frame* get_buddy_frame(Buddy_manager const* const bman, Frame const * const frame, uint8_t order) {
    return get_buddy_frame_by_idx(bman, get_frame_idx(bman, frame), order);
}


static inline bool is_frame_your_buddy(Frame const * const you, Frame const * const buddy) {
    return you->order == buddy->order;
}


Buddy_manager* buddy_init(Buddy_manager* const bman, uint32_t memory_size) {
    size_t frame_nr = memory_size / FRAME_SIZE;

    assert(bman != NULL);
    assert(memory_size != 0);
    assert(frame_nr != 0);

    // TODO: address check
    Frame* frames = malloc(sizeof(Frame) * frame_nr);
    if (frames == NULL) {
        return NULL;
    }

    printf("frames      = %p\n", frames);

    bman->frame_pool        = frames;
    bman->total_frame_nr    = frame_nr;
    for (uint8_t i = 0; i < BUDDY_SYSTEM_MAX_ORDER; ++i) {
        bman->free_frame_nr[i] = 0;
        elist_init(bman->frames + i);
    }

    size_t n = frame_nr;
    uint8_t order = BUDDY_SYSTEM_MAX_ORDER;
    Frame* itr = frames;
    do {
        --order;
        size_t o_nr = BUDDY_SYSTEM_ORDER_NR(order);
        while (n != 0 && o_nr <= n) {
            itr->order = order;
            itr->status = FRAME_STATE_FREE;

            elist_insert_next(&bman->frames[order], &itr->list);
            ++(bman->free_frame_nr[order]);

            itr += o_nr;    /* 次のフレームへ */
            n -= o_nr;      /* 取ったフレーム分を引く */
        }
    } while (0 < order);

    size_t s = 0;
    for (size_t i = 0; i < BUDDY_SYSTEM_MAX_ORDER; i++) {
        elist_foreach(Frame*, itr, bman->frames + i) {
            s += (FRAME_SIZE * BUDDY_SYSTEM_ORDER_NR(i));
        }
    }
    assert(s == memory_size);

    return bman;
}


void buddy_destruct(Buddy_manager* const bman) {
    free(bman->frame_pool);
    memset(bman, 0, sizeof(Buddy_manager));
}


Frame* buddy_get_frames(Buddy_manager* const bman, uint8_t request_order) {
    assert(bman != NULL);
    Elist* frames = bman->frames;

    uint8_t order = request_order;
    while (order < BUDDY_SYSTEM_MAX_ORDER) {
        Elist* l = &frames[order];
        if (elist_is_empty(l) == false) {
            --bman->free_frame_nr[order];
            Frame* rm_frame = elist_get_frame(elist_remove(l->next));

            /* 要求オーダーよりも大きいオーダーからフレームを取得した場合、余分なフレームを繋ぎ直す. */
            while (request_order < order--) {
                Frame* bf = get_buddy_frame(bman, rm_frame, order); /* 2分割 */
                elist_insert_next(&frames[order], &bf->list);       /* バディを一つしたのオーダーのリストへ接続 */
                ++bman->free_frame_nr[order];
            }

            return rm_frame;
        }
        /* requested order is NOT found. */

        ++order;
    }

    /* Error */
    return NULL;
}

static inline void print_separator(void) {
    puts("================================================================================");
}


static inline void buddy_print(Buddy_manager* const bman) {
    print_separator();
    printf("Total Memory Size: %ld KB\n", (bman->total_frame_nr * FRAME_SIZE) / 1024);
    printf("Total Frame: %ld\n", bman->total_frame_nr);
    for (int i = 0; i < BUDDY_SYSTEM_MAX_ORDER; i++) {
        printf("  Order %02d\n", i);

        size_t n = bman->free_frame_nr[i];
        if (n == 0) {
            printf("    No frame\n");
        } else {
            printf("    %ld frame\n", n);
        }

        elist_foreach(Frame*, itr, &bman->frames[i]) {
            printf("    idx: %5ld, addr: 0x%lx\n", get_frame_idx(bman, itr), get_frame_addr(bman, itr));
        }
    }
    print_separator();
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

    Elist dummy;
    Elist* dummy_head = elist_init(&dummy);
    MIN_UNIT_ASSERT("ERROR: elist_init is wrong.", NULL != dummy_head);

    for (size_t i = 0; i < nr; i++) {
        elist_insert_next(dummy_head, &(n[i].list));
        MIN_UNIT_ASSERT("ERROR: elist_insert_next is wrong.", dummy_head->next == (Elist*)&n[i]);
    }

    int cnt = 0;
    elist_foreach (struct number*, i, dummy_head) {
        i->num = cnt++;
    }

    cnt = 0;
    elist_foreach (struct number*, i, dummy_head) {
        MIN_UNIT_ASSERT("ERROR: elist_foreach is wrong.", i->num == cnt++);
    }


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
    printf("============================================================\n\n");
    printf("sizeof(Buddy_manager) = %lu Byte\n", sizeof(Buddy_manager));

    Buddy_manager bman;
    Buddy_manager* p = &bman;

    if (buddy_init(p, FRAME_SIZE * (1 + 2 + 4 + 8 + 16 + 32 + 64 + 128 + 256 + 512 + 1024)) == NULL) {
        return EXIT_FAILURE;
    }
    buddy_print(p);

    buddy_get_frames(p, 0);
    buddy_get_frames(p, 0);
    buddy_print(p);

    buddy_destruct(p);

    return EXIT_SUCCESS;
}
