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
    return ((x != 0) && ((x & complement_2(x)) == x));
}


static inline uintptr_t align_address(uintptr_t addr, size_t align_size) {
    assert(is_power_of_2(align_size));
    return (((addr & (align_size - 1)) == 0)) ? (addr) : ((addr & complement_2(align_size)) + align_size);
}



#endif
