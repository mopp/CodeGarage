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


static inline bool is_power_of_2(size_t x) {
    return ((x != 0) && ((x & (~x + 1)) == x));
}


static inline uintptr_t align_address(uintptr_t addr, size_t align_size) {
    assert(is_power_of_2(align_size));
    return (((addr & (align_size - 1)) == 0)) ? (addr) : ((addr & align_size) + align_size);
}



#endif
