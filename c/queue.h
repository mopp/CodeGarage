/**
 * @file queue.c
 * @brief queue header.
 * @author mopp
 * @version 0.1
 * @date 2014-04-24
 */

#ifndef _QUEUE_H
#define _QUEUE_H


#include "doubly_circularly_linked_list.h"

struct queue {
    List* list;
    bool is_data_type_pointer;
};
typedef struct queue Queue;


extern Queue* init_queue(Queue*, size_t, release_func, bool);
extern bool is_queue_empty(Queue const*);
extern void* get_queue_first(Queue*);
extern void delete_queue_first(Queue*);
extern void* enqueue(Queue*, void*);
extern void* dequeue(Queue*);
extern void destruct_queue(Queue*);
extern size_t get_queue_size(Queue const*);


#endif
