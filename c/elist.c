#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct elist {
    struct elist* next;
    struct elist* prev;
} Elist;


#define elist_derive(type, member_name, ptr) \
    (type*)((uintptr_t)(ptr) - offsetof(type, member_name))

#define elist_foreach(type, lv, i, l) \
    for (type* i = elist_derive(type, lv, (l)->next); (&i->lv != (l)); i = elist_derive(type, lv, i->lv.next))


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


static inline bool elist_is_empty(Elist const* const n) {
    return (n->next == n->prev) && (n == n->next);
}


struct frame {
    Elist list;
    size_t number;
    bool is_free;
};
typedef struct frame Frame;

struct frame_manager {
    Elist free_frames;
    Elist used_frames;
};
typedef struct frame_manager FrameManager;


int main(void)
{
    size_t const SIZE = 32;
    Frame* frames = malloc(sizeof(Frame) * SIZE);
    FrameManager* f_man = malloc(sizeof(FrameManager));

    elist_init(&f_man->free_frames);
    elist_init(&f_man->used_frames);

    for (size_t i = 0; i < SIZE; ++i) {
        Frame* p = frames + i;
        p->number = i;

        elist_init(&p->list);

        if ((i & 1) == 0) {
            p->is_free = false;
            elist_insert_prev(&f_man->used_frames, &p->list);
        } else {
            p->is_free = true;
            elist_insert_prev(&f_man->free_frames, &p->list);
        }
    }

    elist_foreach(Frame, list, item, &f_man->used_frames) {
        printf("%zd, ", item->number);
    }
    putchar('\n');

    elist_foreach(Frame, list, item, &f_man->free_frames) {
        printf("%zd, ", item->number);
    }
    putchar('\n');

    return 0;
}
