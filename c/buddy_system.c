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

#define FRAME_SIZE 0x1000                        /* frame size is 4 KB. */
#define MAX_MEMORY_SIZE (4 * 1024 * 1024 * 1024) /* max memory size of x86_32 is 4GB */


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
    Frame frames[BUDDY_SYSTEM_MAX_ORDER];
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


Buddy_manager* init_buddy_system(Buddy_manager* bman, Frame const* const frame, uint32_t frame_nr) {
    bman->free_memory_size  = 0;
    bman->alloc_memory_size = 0;
    for (int i = 0; i < BUDDY_SYSTEM_MAX_ORDER; ++i) {
        elist_init(&bman->frames[i].list);
        bman->free_page_nr[i] = 0;
    }

    uint32_t max_size = frame_nr * FRAME_SIZE;
    for (int i = BUDDY_SYSTEM_MAX_ORDER - 1; 0 <= i; --i) {
        bman->frames[i];
    }

    return bman;
}


int main(void) {
    uint32_t const frame_nr = MAX_MEMORY_SIZE / FRAME_SIZE;
    Frame* const frames = malloc(sizeof(Frame) * frame_nr);
    memset(frames, 0, sizeof(Frame) * frame_nr);
    if (frames == NULL) {
        return EXIT_FAILURE;
    }

    Buddy_manager bman;
    /* init_buddy_system(&bman, MAX_MEMORY_SIZE); */

    free(frames);

    return EXIT_SUCCESS;
}
