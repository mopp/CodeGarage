/**
 * @file memory_dump.c
 * @brief formated memory dump function.
 * @author mopp
 * @version 0.1
 * @date 2014-04-24
 */
#include <stdint.h>
#include <stdio.h>

#define BASE 16
#define MOD_16(x) (x & 0x0F)


void dump_memory_hex(uintptr_t const buf, size_t const size) {
    uint8_t const *const b = (uint8_t const * const)buf;
    uint8_t ascii[BASE + 1] = {[BASE] = '\0'};

    char dummy[20];
    int const addr_len = sprintf(dummy, "%lX", buf);

    printf("%*c    +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F |  -- ASCII --\r\n", addr_len, ' ');
    printf("%*c    --+--+--+--+--+--+--+--+--+--+--+--+--+--+--+---+----------------\r\n", addr_len, ' ');

    for (size_t i = 0; i < size; ++i) {
        uint8_t const t = MOD_16(i);
        if (t == 0) {
            printf("%*p:", addr_len, &b[i << 4]);
        }

        uint8_t c = b[i];
        printf(" %02X", c);
        ascii[t] = (c < 0x20 || 0x7F <= c) ? '.' : c;

        if (t == BASE - 1) {
            printf(" |%s\n", ascii);
        }
    }

    uint8_t const t = MOD_16(size);
    if (t != 0) {
        ascii[t] = '\0';
        printf("%*c |%s\n", (BASE - t) * 3, ' ', ascii);
    }
}


int main(void) {
    uint32_t i = 0xFFFFFFFF;

    dump_memory_hex((uintptr_t)(&i), 100);

    return 0;
}
