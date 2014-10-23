#include <stdio.h>
#include <time.h>
#include <sys/time.h>
#include <stddef.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <ctype.h>
#include <limits.h>


static double gettimeofday_sec(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return tv.tv_sec + tv.tv_usec * 1e-6;
}


static inline void* memcpy0(void* restrict buf1, const void* restrict buf2, size_t n) {
    char* p1 = (char*)buf1;
    char const* p2 = (char const*)buf2;

    while (0 < n--) {
        *p1 = *p2;
        ++p1;
        ++p2;
    }

    return buf1;
}


static inline void* memcpy1(void* restrict b1, const void* restrict b2, size_t n) {
    uint8_t* p1 = b1;
    uint8_t const* p2 = b2;

    size_t byte_nr = n % sizeof(uint32_t);
    n -= byte_nr;
    n /= sizeof(uint32_t);
    while (0 < byte_nr--) {
        *p1 = *p2;
        ++p1;
        ++p2;
    }

    uint32_t* dw1 = (uint32_t*)p1;
    uint32_t const* dw2 = (uint32_t const*)p2;
    while (0 < n--) {
        *dw1 = *dw2;
        ++dw1;
        ++dw2;
    }

    return b1;
}


static inline void* memcpy2(void* restrict b1, const void* restrict b2, size_t n) {
    __asm__ volatile(
            "cld        \n"
            "rep movsb  \n"
            :
            : "S"(b2), "D"(b1), "c"(n)
            : "memory", "%esi", "%edi", "%ecx"
    );

    return b1;
}


static inline void* memcpy3(void* restrict b1, const void* restrict b2, size_t n) {
    uint8_t* restrict p1 = b1;
    uint8_t const* restrict p2 = b2;
    size_t nr, t;

    __asm__ volatile("cld");

    nr = n / 4;
    if (nr != 0) {
        __asm__ volatile(
                "rep movsd  \n"
                :
                : "S"(p2), "D"(p1), "c"(nr)
                : "memory", "%esi", "%edi", "%ecx"
        );
        t = nr * 4;
        n -= t;
        p1 += t;
        p2 += t;
    }


    nr = n / 2;
    if (nr != 0) {
        __asm__ volatile(
                "rep movsw   \n"
                :
                : "S"(p2), "D"(p1), "c"(nr)
                : "memory", "%esi", "%edi", "%ecx"
        );
        t = nr * 2;
        n -= t;
        p1 += t;
        p2 += t;
    }

    __asm__ volatile(
            "rep movsb   \n"
            :
            : "S"(p2), "D"(p1), "c"(n)
            : "memory", "%esi", "%edi", "%ecx"
    );

    return b1;
}


typedef void* (*memcpy_f)(void* restrict, const void* restrict, size_t);
static double worker(memcpy_f f, void* restrict b1, void* restrict b2, size_t n) {
    static int cnt = 0;
    double t1, t2;

    /* init randam data */
    srand((unsigned)time(NULL));
    for (int i = 0; i < n; i++) {
        *(char*)b2 = (char)(rand() / CHAR_MAX);
    }

    t1 = gettimeofday_sec();
    f(b1, b2, n);
    t2 = gettimeofday_sec();

    printf("No.%d\n", cnt++);
    if (memcmp(b1, b2, n) != 0) {
        printf("validation failed\n");
    } else {
        printf("    %f\n", t2 - t1);
    }

    return t2 - t1;
}


int main(void) {
    enum  {
       n = 223810,
    };
    // char* b1 = malloc(sizeof(char) * n);
    // char* b2 = malloc(sizeof(char) * n);
    /* size_t n = 100; */
    char* b1[n];
    char* b2[n];

    worker(memcpy0, b1, b2, n);
    worker(memcpy1, b1, b2, n);
    worker(memcpy2, b1, b2, n);
    worker(memcpy3, b1, b2, n);

    /* free(b1); */
    /* free(b2); */

    return 0;
}
