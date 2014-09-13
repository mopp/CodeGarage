/**
 * @file buddy_system.c
 * @brief This is CUI simulater of Buddy System algorithm for x86_32.
 * @author mopp
 * @version 0.1
 * @date 2014-09-10
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdalign.h>
#include <string.h>


/* The number of frame  : 1 2 4 8 16 32 64 128 256 512 1024 */
/* Order in buddy system: 0 1 2 3  4  5  6   7   8   9   10 */
#define BUDDY_SYSTEM_MAX_ORDER 11

#define FRAME_SIZE 0x1000U                        /* frame size is 4 KB. */
#define MAX_MEMORY_SIZE (0xffffffffU) /* max memory size of x86_32 is 4GB */


typedef struct elist {
    struct elist* next;
    struct elist* prev;
} Elist;


struct frame {
    Elist list;
    uint8_t status;
    uint8_t exponent;
};
typedef struct frame Frame;


enum frame_constants {
    FRAME_STATE_FREE = 0,
    FRAME_STATE_ALLOC,
};


struct buddy_manager {
    Frame* frame_pool;
    Frame* frames[BUDDY_SYSTEM_MAX_ORDER];
    uint32_t free_page_nr[BUDDY_SYSTEM_MAX_ORDER];
    uint32_t free_memory_size;
    uint32_t alloc_memory_size;
};
typedef struct buddy_manager Buddy_manager;



static inline Elist* elist_init(Elist* l) {
    l->next = l;
    l->prev = l;
    return l;
}


static inline Elist* elist_insert_next(Elist* l, Elist* new) {
    new->next = l->next;
    new->prev = l;
    new->next->prev = new;
    l->next = new;

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


Buddy_manager* init_buddy_system(Buddy_manager* bman, uint32_t memory_size) {
    uint32_t const frame_nr = memory_size / FRAME_SIZE;
    Frame* const frames = malloc(sizeof(Frame) * frame_nr);
    memset(frames, 0, sizeof(Frame) * frame_nr);
    if (frames == NULL) {
        return NULL;
    }

    bman->free_memory_size  = 0;
    bman->alloc_memory_size = 0;
    for (int i = 0; i < BUDDY_SYSTEM_MAX_ORDER; ++i) {
        elist_init(&bman->frames[i]->list);
        bman->free_page_nr[i] = 0;
    }

    uint32_t max_size = frame_nr * FRAME_SIZE;
    printf("max_size = %u KB\n", max_size / 1024);
    printf("frame_nr = %u\n", frame_nr);
    for (int i = BUDDY_SYSTEM_MAX_ORDER - 1; 0 <= i; --i) {
        uint32_t order_size = (1 << i);
        int cnt = 0;
        while (order_size <= frame_nr) {
            frame_nr -= order_size;
            ++cnt;
        }
        printf("order = %2u ", i);
        printf("order size = %4u - %d\n", order_size, cnt);
    }

    return bman;
}


int main(void) {
    Buddy_manager bman;
    init_buddy_system(&bman, MAX_MEMORY_SIZE);

    return EXIT_SUCCESS;
}
