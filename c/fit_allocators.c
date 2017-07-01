#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

struct block {
    uintptr_t addr;
    size_t size;
    bool is_alloc;
};
typedef struct block Block;

static const size_t TOTAL_MEMORY_SIZE_BYTE      = 1024 * 8;
static const size_t BLOCK_MEMORY_UNIT_SIZE_BYTE = 256;
static const size_t BLOCK_COUNT = TOTAL_MEMORY_SIZE_BYTE / BLOCK_MEMORY_UNIT_SIZE_BYTE;

struct block_strage {
    Block blocks[BLOCK_COUNT];
};
typedef struct block_strage BlockStrage;

static BlockStrage* block_strage_create()
{
    BlockStrage* bs = malloc(sizeof(BlockStrage));

    uintptr_t addr =  (uintptr_t)malloc(TOTAL_MEMORY_SIZE_BYTE);
    for (size_t i = 0; i < BLOCK_COUNT; i++) {
        Block* b    = &bs->blocks[i];
        b->addr     = addr;
        b->size     = BLOCK_MEMORY_UNIT_SIZE_BYTE;
        b->is_alloc = false;
        addr += BLOCK_MEMORY_UNIT_SIZE_BYTE;
    }

    return bs;
}


void block_strage_destroy(BlockStrage* bs)
{
    free((void*)bs->blocks[0].addr);
    free(bs);
}

size_t block_strage_free_size(BlockStrage const* bs)
{
    size_t sum = 0;
    for (size_t i = 0; i < BLOCK_COUNT; i++) {
        Block const* b = &bs->blocks[i];

        if (b->is_alloc == false) {
            sum += b->size;
        }
    }

    return sum;
}


void block_strage_dump(char const* tag, BlockStrage const* bs)
{
    printf("%s\n", tag);
    printf("  Free size: %zd byte\n", block_strage_free_size(bs));
    printf("  Used size: %zd byte\n", TOTAL_MEMORY_SIZE_BYTE - block_strage_free_size(bs));
}


void* alloc_firstfit(BlockStrage* bs, size_t size)
{
    for (size_t i = 0; i < BLOCK_COUNT; i++) {
        Block* b = &bs->blocks[i];

        // TODO alloc continuous blocks.
        if ((b->is_alloc == false) && (size <= b->size)) {
            b->is_alloc = true;
            return (void*)b->addr;
        }
    }

    return NULL;
}


int main(void)
{
    BlockStrage* bs_first_fit = block_strage_create();
    alloc_firstfit(bs_first_fit, 100);
    block_strage_dump("FirstFit", bs_first_fit);
    /* BlockStrage* next_fit = block_strage_create(); */
    alloc_firstfit(bs_first_fit, 100);

    return 0;
}
