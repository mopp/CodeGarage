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


#endif
