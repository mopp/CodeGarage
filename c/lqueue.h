/**
 * @file lqueue.h
 * @brief List queue header.
 * @author mopp
 * @version 0.1
 * @date 2014-04-24
 */

#ifndef _L_QUEUE_H_
#define _L_QUEUE_H_



#include "dlist.h"

struct lqueue {
    Dlist* list;
};
typedef struct lqueue Lqueue;

extern Lqueue* lqueue_init(Lqueue*, size_t, release_func);
extern bool lqueue_is_empty(Lqueue const*);
extern void* lqueue_get_first(Lqueue*);
extern void lqueue_delete_first(Lqueue*);
extern void* lqueue_insert(Lqueue*, void*);
extern void lqueue_destruct(Lqueue*);
extern size_t lqueue_get_size(Lqueue const*);


#define aqueue_get(type, q) (*(type*)lqueue_get_first(q))



#endif
