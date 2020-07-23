#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

static void* top;
static void* bottom;
void* my_malloc(size_t);

int main(int argc, char const* argv[])
{
    // Initialize memory allocator.
    size_t memory_capacity = 512 * 1024;
    top = malloc(memory_capacity);
    void* origin = top;
    bottom = top + memory_capacity;
    printf("top: %p, bottom: %p, remain: %zx\n", top, bottom, bottom - top);

    char* about = my_malloc(512);
    *about = 'a';

    printf("top: %p, bottom: %p, remain: %zx, got addr: %p\n", top, bottom, bottom - top, about);

    char* mopp = my_malloc(2048);
    printf("top: %p, bottom: %p, remain: %zx, got addr: %p\n", top, bottom, bottom - top, mopp);

    mopp = my_malloc(512 * 1024 * 1024);
    printf("top: %p, bottom: %p, remain: %zx, got addr: %p\n", top, bottom, bottom - top, mopp);

    // Destruct.
    free(origin);
    printf("Completed\n");

    return 0;
}

void* my_malloc(size_t request_size_byte) {
    if ((bottom - top) < (request_size_byte)) {
        return NULL;
    }

    uintptr_t addr = (uintptr_t)top;

    // Shift address for the next allocation.
    top += request_size_byte;

    return (void*)addr;
}
