#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(void) {
    #define SIZE 8192
    char* const msg = malloc(SIZE);

    size_t count = 0;
    do {
        memcpy((void*)(msg + count), "y\n", 2);
    } while ((count += 2) < SIZE);

    while(write(STDOUT_FILENO, msg, SIZE)) {}

    return 0;
}
