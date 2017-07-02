#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

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



static BlockStrage* block_strage_create();
static void block_strage_destroy(BlockStrage*);
static size_t block_strage_free_size(BlockStrage const*);
static void block_strage_dump(char const*, BlockStrage const*);
static void* alloc_firstfit(BlockStrage*, size_t);


int main(void)
{
    BlockStrage* bs_first_fit = block_strage_create();
    /* BlockStrage* next_fit = block_strage_create(); */

    alloc_firstfit(bs_first_fit, 100);
    alloc_firstfit(bs_first_fit, 100);
    block_strage_dump("FirstFit", bs_first_fit);
    assert(block_strage_free_size(bs_first_fit) == (TOTAL_MEMORY_SIZE_BYTE - BLOCK_MEMORY_UNIT_SIZE_BYTE * 2));

    block_strage_destroy(bs_first_fit);

    return 0;
}


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


static void block_strage_destroy(BlockStrage* bs)
{
    free((void*)bs->blocks[0].addr);
    free(bs);
}

static size_t block_strage_free_size(BlockStrage const* bs)
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


static void block_strage_dump(char const* tag, BlockStrage const* bs)
{
    printf("%s\n", tag);
    printf("  Free size: %zd byte\n", block_strage_free_size(bs));
    printf("  Used size: %zd byte\n", TOTAL_MEMORY_SIZE_BYTE - block_strage_free_size(bs));
}


static void* alloc_firstfit(BlockStrage* bs, size_t size)
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
