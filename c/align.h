/**
 * @file align.h
 * @brief align header.
 * @author mopp
 * @version 0.1
 * @date 2014-09-24
 */


#ifndef _ALIGN_H_
#define _ALIGN_H_



#include <assert.h>
#include <stdint.h>
#include <stdbool.h>


static inline size_t complement_2(size_t x) {
    return (~x + 1);
}


static inline bool is_power_of_2(size_t x) {
    return ((x != 0) && ((x - 1u) & x) == 0);
}


static inline uintptr_t align_address(uintptr_t addr, size_t align_size) {
    assert(is_power_of_2(align_size));
    size_t const a = align_size - 1;
    return ((size_t)(addr + a) & ~(a));
}


static inline size_t align_up(size_t x, size_t a) {
    return (x + (a - 1u)) & ~(a - 1u);
}


static inline size_t align_down(size_t x, size_t a) {
    return x & ~(a - 1u);
}



#endif
