#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <sys/time.h>


static double gettimeofday_sec(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return tv.tv_sec + tv.tv_usec * 1e-6;
}


static void* memset0(void* s, int c, size_t n) {
    if (n != 0) {
        register uintptr_t addr = (uintptr_t)s;
        register uintptr_t t = addr + n;
        if (((addr | n) & 1) == 0) {
            for (unsigned int val = (uint8_t)c | (uint8_t)c << 8; addr < t; addr += sizeof(uint16_t)) {
                *(uint16_t*)addr = val;
            }
        }
        for (; addr < t; ++addr)
            *(uint8_t*)addr = c;
    }
    return s;
}


static void* memset1(void* buf, int ch, size_t n) {
    unsigned char* t = (unsigned char*)buf;

    while (0 < n--) {
        *t++ = (unsigned char const)ch;
    }

    return buf;
}


static void* memset2(void* s, int c, size_t n) {
    register uintptr_t itr = (uintptr_t)s;
    register uintptr_t end = itr + n;
    uint8_t const uc = (uint8_t const)c;
    if (((itr | n) & 0x3) == 0) {
        register uint32_t v = (uint32_t)((uc << 24) | (uc << 16) | (uc << 8) | uc);
        while (itr < end) {
            *(uint32_t*)(itr) = v;
            itr += sizeof(uint32_t);
        }
    } else if (((itr | n) & 0x1) == 0) {
        /* word */
        register uint16_t v = (uint16_t)((uc << 8) | uc);
        while (itr < end) {
            *(uint16_t*)(itr) = v;
            itr += sizeof(uint16_t);
        }
    }

    /* byte */
    while (itr < end) {
        *(uint8_t*)(itr++) = uc;
    }

    return s;
}


static void* memset3(void* s, register int c, size_t n) {
    register uintptr_t itr = (uintptr_t)s;
    register uintptr_t end = itr + n;

    size_t mod = n & 0x3;
    end = itr + mod;
    while (itr < end) {
        *(uint8_t*)(itr++) = (uint8_t)c;
    }

    end = itr + (n - mod);

    register uint32_t v = (uint32_t)((c << 24) | (c << 16) | (c << 8) | c);
    while (itr < end) {
        while (itr < end) {
            *(uint32_t*)(itr) = v;
            itr += sizeof(uint32_t);
        }
    }

    return s;
}


static void* memset4(void* s, register int c, size_t n) {
    register uintptr_t itr = (uintptr_t)s;
    register uintptr_t end = itr + n;

    for (register uint32_t v = (uint32_t)((c << 24) | (c << 16) | (c << 8) | c); itr < end; itr += sizeof(uint32_t)) {
        *(uint32_t*)(itr) = v;
    }

    for (register uint16_t v = (uint16_t)((c << 8) | c); itr < end; itr += sizeof(uint16_t)) {
        *(uint16_t*)(itr) = v;
    }

    for (register uint8_t v = (uint8_t)c; itr < end; itr += sizeof(uint8_t)) {
        *(uint8_t*)(itr) = v;
    }


    return s;
}


static bool validate(register uint8_t* s, register uint8_t c, size_t n) {
    register uint8_t* e = s + n;

    while (s < e) {
        if (*s++ != c) {
            printf("%p\n", s);
            return false;
        }
    }

    return true;
}


typedef void* (*memset_f)(void*, int, size_t);


static double worker(memset_f f, int c, size_t n) {
    double t1, t2;

    void* s = malloc(n);

    t1 = gettimeofday_sec();
    f(s, c, n);
    t2 = gettimeofday_sec();

    if (validate(s, c & 0xff, n) == false) {
        printf("validation false\n");
    }

    printf("    %f\n", t2 - t1);

    free(s);

    return t2 - t1;
}


static void worker2(memset_f f) {
    size_t const n = 10;
    size_t sizes[10] = {100, 64 * 1024, 29 * 1024 * 1024, 43 * 1024, 1016201, 32 * 1024 * 1024, 9874389, 64 * 1024, 4096, 4096 * 1024};
    double sum = 0;

    for (size_t i = 0; i < n; i++) {
        sum += worker(f, 0xff & i, sizes[i]);
    }

    printf("  time avg. %f\n", sum / n);
}


static void worker3(memset_f f, size_t n) {
    size_t const times = 10000000;
    double sum = 0;

    double t1, t2;

    t1 = gettimeofday_sec();
    for (size_t i = 0; i < times; i++) {
        sum += worker(f, 6, n);
    }
    t2 = gettimeofday_sec();

    printf("  time.     %f\n", t2 - t1);
}


int main(void) {
    memset_f fs[] = {memset2, memset3, memset4};

    for (size_t i = 0; i < 3; i++) {
        worker2(fs[i]);
    }

    return 0;
}

