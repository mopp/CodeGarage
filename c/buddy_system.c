/**
 * @file buddy_system.c
 * @brief This is CUI simulater of Buddy System allocater for x86_32.
 * @author mopp
 * @version 0.1
 * @date 2014-09-23
 */

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "minunit.h"


/* Order in buddy system: 0 1 2 3  4  5  6   7   8   9   10 */
/* The number of frame  : 1 2 4 8 16 32 64 128 256 512 1024 */
#define BUDDY_SYSTEM_MAX_ORDER (10 + 1)
#define BUDDY_SYSTEM_ORDER_NR(order) (1U << (order))

/* frame size is 4 KB in x86_32. */
#define FRAME_SIZE 0x1000U

#define ORDER_FRAME_SIZE(order) (BUDDY_SYSTEM_ORDER_NR(order) * FRAME_SIZE)

#define TO_KB(x) (x >> 11)


/* Equipment list */
typedef struct elist {
    struct elist* next;
    struct elist* prev;
} Elist;


struct frame {
    Elist list;
    uint8_t status;
    uint8_t order;
};
typedef struct frame Frame;


enum frame_constants {
    FRAME_STATE_FREE = 0,
    FRAME_STATE_ALLOC,
};


/* Buddy system manager. */
struct buddy_manager {
    Frame* frame_pool;                            /* 管理用の全フレーム */
    size_t total_frame_nr;                        /* マネージャの持つ全フレーム数 */
    size_t free_frame_nr[BUDDY_SYSTEM_MAX_ORDER]; /* 各オーダーの空きフレーム数 */
    Elist frames[BUDDY_SYSTEM_MAX_ORDER];         /* 各オーダーのリスト先頭要素(ダミー), 実際のデータはこのリストのnext要素から始まる. */
};
typedef struct buddy_manager Buddy_manager;


#define elist_get_element(type, list) (type)(list)

#define elist_foreach(type, var, list) \
    for (type var = elist_get_element(type, (list)->next); (uintptr_t)var != (uintptr_t)(list); var = elist_get_element(type, ((Elist*)var)->next))


static inline Elist* elist_init(Elist* l) {
    l->next = l;
    l->prev = l;
    return l;
}


static inline Elist* elist_insert_next(Elist* l, Elist* new) {
    new->next = l->next;
    new->prev = l;
    l->next = new;
    new->next->prev = new;

    return l;
}


// static inline Elist* elist_insert_prev(Elist* l, Elist* new) {
//     return elist_insert_next(l->prev, new);
// }


static inline Elist* elist_remove(Elist* n) {
    Elist* next = n->next;
    Elist* prev = n->prev;
    prev->next = next;
    next->prev = prev;

    return elist_init(n);
}


static inline bool elist_is_empty(Elist* n) {
    return (n->next == n->prev) && (n == n->next);
}


static inline Frame* elist_get_frame(Elist const* const l) {
    return (Frame*)l;
}


/**
 * @brief フレームのindexを求める..
 * @param bman  フレームの属するマネージャ.
 * @param frame indexを求めるフレーム.
 * @return フレームのindex.
 */
static inline size_t get_frame_idx(Buddy_manager const* const bman, Frame const* const frame) {
    assert(bman != NULL);
    assert(frame != NULL);
    assert((uintptr_t)bman->frame_pool <= (uintptr_t)frame);
    return ((uintptr_t)frame - (uintptr_t)bman->frame_pool) / sizeof(Frame);
}


/**
 * @brief フレームのアドレスを求める..
 * @param bman  フレームの属するマネージャ.
 * @param frame アドレスを求めるフレーム.
 * @return フレームのアドレス.
 */
uintptr_t get_frame_addr(Buddy_manager const* const bman, Frame const* const frame) {
    assert(bman != NULL);
    assert(frame != NULL);
    return get_frame_idx(bman, frame) * FRAME_SIZE;
}


/**
 * @brief バディのアドレスが2の累乗であることとxorを利用して、フレームのバディを求める関数.
 *          xor は そのビットが1であれば0に、0であれば1にする.
 *          ---------------------
 *          | Buddy A | Buddy B |
 *          ---------------------
 *          上図において、引数がBuddy Aなら足して、Buddy Bを求める.
 *          Buddy Bなら引いて、Buddy Aを求めるという処理になる.
 *          また、オーダーが1の時、要素0番のバディは要素2番である.
 *          0 + (1 << 1) = 2
 * @param bman  フレームの属するマネージャ.
 * @param frame バディを求めたいフレーム.
 * @param order バディを求めるオーダー.
 * @return バディが存在しない場合NULLが返る.
 */
static inline Frame* get_buddy_frame(Buddy_manager const* const bman, Frame const* const frame, uint8_t order) {
    Frame* p = bman->frame_pool;
    Frame* f = p + (get_frame_idx(bman, frame) ^ BUDDY_SYSTEM_ORDER_NR(order));
    return ((p + bman->total_frame_nr) <= f) ? NULL : f;
}


/**
 * @brief バディマネージャを初期化.
 * @param bman        初期化対象
 * @param memory_size バディマネージャの管理するメモリーサイズ.
 * @return 初期化出来なかった場合NULL, それ以外は引数のマネージャが返る.
 */
Buddy_manager* buddy_init(Buddy_manager* const bman, size_t memory_size) {
    size_t frame_nr = memory_size / FRAME_SIZE;

    assert(bman != NULL);
    assert(memory_size != 0);
    assert(frame_nr != 0);

    /* TODO: address align check. */
    Frame* frames = malloc(sizeof(Frame) * frame_nr);
    assert(((uintptr_t)frames & 0x01) == 0);
    if (frames == NULL) {
        return NULL;
    }

    /* 確保した全フレーム初期化. */
    Frame* p = frames;
    Frame* end = frames + frame_nr;
    do {
        p->status = FRAME_STATE_FREE;
    } while (++p <= end);

    /* マネージャを初期化 */
    bman->frame_pool = frames;
    bman->total_frame_nr = frame_nr;
    for (uint8_t i = 0; i < BUDDY_SYSTEM_MAX_ORDER; ++i) {
        bman->free_frame_nr[i] = 0;
        elist_init(bman->frames + i);
    }

    /* フレームを大きいオーダーからまとめてリストを構築. */
    size_t n = frame_nr;
    uint8_t order = BUDDY_SYSTEM_MAX_ORDER;
    Frame* itr = frames;
    do {
        --order;
        size_t o_nr = BUDDY_SYSTEM_ORDER_NR(order);
        while (n != 0 && o_nr <= n) {
            /* フレームを現在オーダのリストに追加. */
            itr->order  = order;
            itr->status = FRAME_STATE_FREE;
            elist_insert_next(&bman->frames[order], &itr->list);
            ++(bman->free_frame_nr[order]);

            itr += o_nr; /* 次のフレームへ. */
            n -= o_nr;   /* 取ったフレーム分を引く. */
        }
    } while (0 < order);

    return bman;
}


/**
 * @brief バディマネージャを破棄.
 * @param bman        破棄対象
 */
void buddy_destruct(Buddy_manager* const bman) {
    free(bman->frame_pool);
    memset(bman, 0, sizeof(Buddy_manager));
}


/**
 * @brief 指定オーダーのフレームを確保する.
 * @param bman          確保先のフレームを持つマネージャ.
 * @param request_order 確保するオーダー.
 * @return 確保出来なかった場合NULLが返る.
 */
Frame* buddy_alloc_frames(Buddy_manager* const bman, uint8_t request_order) {
    assert(bman != NULL);
    Elist* frames = bman->frames;

    /* O(10 * 10) ? */
    uint8_t order = request_order;
    while (order < BUDDY_SYSTEM_MAX_ORDER) {
        Elist* l = &frames[order];
        if (elist_is_empty(l) == false) {
            --bman->free_frame_nr[order];
            Frame* rm_frame = elist_get_frame(elist_remove(l->next));
            rm_frame->order = request_order;
            rm_frame->status = FRAME_STATE_ALLOC;

            /* 要求オーダーよりも大きいオーダーからフレームを取得した場合、余分なフレームを繋ぎ直す. */
            while (request_order < order--) {
                Frame* bf = get_buddy_frame(bman, rm_frame, order); /* 2分割 */
                bf->order = order;                                  /* 分割したのでバディのオーダーを設定.これをやらないと解放時に困る. */
                elist_insert_next(&frames[order], &bf->list);       /* バディを一つしたのオーダーのリストへ接続 */
                ++bman->free_frame_nr[order];
            }

            return rm_frame;
        }
        /* requested order is NOT found. */

        ++order;
    }

    /* Error */
    return NULL;
}


/**
 * @brief フレームを解放する.
 * @param bman フレームの返却先マネージャ.
 * @param ffs  解放するフレーム.
 */
void buddy_free_frames(Buddy_manager* const bman, Frame* ffs) {
    Frame* bf;
    uint8_t order = ffs->order;

    // 開放するフレームのバディが空きであれば、2つを合わせる.
    while ((order < (BUDDY_SYSTEM_MAX_ORDER - 1)) && ((bf = get_buddy_frame(bman, ffs, order)) != NULL) && (order == bf->order) && (bf->status == FRAME_STATE_FREE)) {
        elist_remove(&bf->list);
        --bman->free_frame_nr[bf->order];
        ++order;
    }

    ++bman->free_frame_nr[order];
    ffs->order = order;
    ffs->status = FRAME_STATE_FREE;
    elist_insert_next(&bman->frames[order], &ffs->list);
}


/**
 * @brief マネージャ管理下の空きメモリ容量を求める.
 * @param bman 求める対象のマネージャ.
 * @return 空きメモリ容量.
 */
size_t buddy_get_free_memory_size(Buddy_manager const* const bman) {
    size_t free_mem_size = 0;
    for (uint8_t i = 0; i < BUDDY_SYSTEM_MAX_ORDER; i++) {
        free_mem_size += (bman->free_frame_nr[i] * ORDER_FRAME_SIZE(i));
    }

    return free_mem_size;
}


/**
 * @brief マネージャ管理下の使用メモリ容量を求める.
 * @param bman 求める対象のマネージャ.
 * @return 使用メモリ容量.
 */
size_t buddy_get_alloc_memory_size(Buddy_manager const* const bman) {
    return (bman->total_frame_nr * FRAME_SIZE) - buddy_get_free_memory_size(bman);
}


/* ==================== Test functions. ==================== */

static char const* test_elist_foreach(void) {
    struct number {
        Elist list;
        int num;
    };

    size_t nr = 10;
    struct number n[nr];

    for (int i = 0; i < nr; i++) {
        n[i].num = i;
    }

    Elist dummy;
    Elist* dummy_head = elist_init(&dummy);
    MIN_UNIT_ASSERT("elist_init is wrong.", NULL != dummy_head);

    for (size_t i = 0; i < nr; i++) {
        elist_insert_next(dummy_head, &(n[i].list));
        MIN_UNIT_ASSERT("elist_insert_next is wrong.", dummy_head->next == (Elist*)&n[i]);
    }

    int cnt = 0;
    elist_foreach(struct number*, i, dummy_head) {
        i->num = cnt++;
    }

    int valid_num[] = {9, 8, 7, 6, 5, 4, 3, 2, 1, 0};
    for (int i = 0; i < nr; i++) {
        MIN_UNIT_ASSERT("elist_foreach is wrong.", n[i].num == valid_num[i]);
    }


    return NULL;
}


static char const* test_get_frame_addr(void) {
    Buddy_manager bman;
    bman.frame_pool = malloc(sizeof(Frame) * 10);

    MIN_UNIT_ASSERT("get_frame_addr is wrong.", 0 == get_frame_addr(&bman, bman.frame_pool));
    MIN_UNIT_ASSERT("get_frame_addr is wrong.", FRAME_SIZE == get_frame_addr(&bman, bman.frame_pool + 1));
    MIN_UNIT_ASSERT("get_frame_addr is wrong.", FRAME_SIZE * 10 == get_frame_addr(&bman, bman.frame_pool + 10));

    free(bman.frame_pool);

    return NULL;
}


static char const* test_get_buddy_frame(void) {
    Buddy_manager bman;
    bman.frame_pool = malloc(sizeof(Frame) * 10);
    bman.total_frame_nr = 10;

    MIN_UNIT_ASSERT("get_buddy_frame is wrong.", &bman.frame_pool[1] == get_buddy_frame(&bman, &bman.frame_pool[0], 0));
    MIN_UNIT_ASSERT("get_buddy_frame is wrong.", &bman.frame_pool[0] == get_buddy_frame(&bman, &bman.frame_pool[1], 0));
    MIN_UNIT_ASSERT("get_buddy_frame is wrong.", &bman.frame_pool[2] == get_buddy_frame(&bman, &bman.frame_pool[0], 1));
    MIN_UNIT_ASSERT("get_buddy_frame is wrong.", &bman.frame_pool[8] == get_buddy_frame(&bman, &bman.frame_pool[0], 3));

    free(bman.frame_pool);

    return NULL;
}


static char const* test_buddy_init(void) {
    size_t memory_size = FRAME_SIZE * 10;
    Buddy_manager bman;
    buddy_init(&bman, memory_size);

    size_t s = 0;
    for (size_t i = 0; i < BUDDY_SYSTEM_MAX_ORDER; i++) {
        elist_foreach(Frame*, itr, &bman.frames[i]) {
            s += ORDER_FRAME_SIZE(i);
        }
    }

    MIN_UNIT_ASSERT("buddy_init is wrong.", s == memory_size);

    return NULL;
}


static char const* test_buddy_alloc_free(void) {
    size_t memory_size = FRAME_SIZE * 1024 * 512;
    Buddy_manager bman;
    buddy_init(&bman, memory_size);

    for (int i = BUDDY_SYSTEM_MAX_ORDER - 1; 0 <= i; --i) {
        while (buddy_alloc_frames(&bman, i & 0xff) != NULL);
    }

    MIN_UNIT_ASSERT("buddy_init is wrong.", 0 == buddy_get_free_memory_size(&bman));
    MIN_UNIT_ASSERT("buddy_init is wrong.", memory_size == buddy_get_alloc_memory_size(&bman));

    return NULL;
}


static char const* all_tests(void) {
    MIN_UNIT_RUN(test_elist_foreach);
    MIN_UNIT_RUN(test_get_frame_addr);
    MIN_UNIT_RUN(test_get_buddy_frame);
    MIN_UNIT_RUN(test_buddy_init);
    MIN_UNIT_RUN(test_buddy_alloc_free);

    return NULL;
}


static inline int do_all_tests(void) {
    MIN_UNIT_RUN_ALL(all_tests);
}


/* ==================== Display functions. ==================== */

static inline void print_separator(void) {
    puts("================================================================================");
}


static inline void print_box(char const * const msg) {
    print_separator();
    printf("%s\n", msg);
    print_separator();
}


static inline void newline(void) {
    putchar('\n');
}


static inline void print_frame_info(Buddy_manager const * const bman, Frame const * const f) {
    uintptr_t s = get_frame_addr(bman, f);
    printf("idx: %5zd, addr: 0x%08zx ~ 0x%08zx\n", get_frame_idx(bman, f), s, s + (FRAME_SIZE * BUDDY_SYSTEM_ORDER_NR(f->order)));
}


static inline void print_buddy_system(Buddy_manager* const bman) {
    size_t total_mem_size = (bman->total_frame_nr * FRAME_SIZE);
    printf("Total Frame       : %zd\n", bman->total_frame_nr);
    printf("Total Memory Size : %zd KB\n", TO_KB(total_mem_size));
    printf("Free Memory Size  : %zd KB\n", TO_KB(buddy_get_free_memory_size(bman)));
    printf("Alloc Memory Size : %zd KB\n", TO_KB(buddy_get_alloc_memory_size(bman)));
    printf("Address region: 0x00000000 ~ 0x%08zx\n", FRAME_SIZE * bman->total_frame_nr);

    for (uint8_t i = 0; i < BUDDY_SYSTEM_MAX_ORDER; i++) {
        printf("  Order %02u\n", i);

        size_t n = bman->free_frame_nr[i];
        if (n == 0) {
            printf("    No frame\n");
        } else {
            printf("    %zd frame\n", n);
        }

        elist_foreach(Frame*, itr, &bman->frames[i]) {
            printf("    ");
            print_frame_info(bman, itr);
            assert(i == itr->order);
        }
    }
}


int main(void) {
    do_all_tests();

    newline();
    print_box("Start Buddy System Simulater");

    Buddy_manager bman;
    Buddy_manager* p = &bman;

    printf("Please input the number of frame : ");
    scanf(" %zd", &p->total_frame_nr);

    p = buddy_init(p, FRAME_SIZE * (p->total_frame_nr));
    if (p == NULL) {
        fprintf(stderr, "*Initialize Buddy System failed*\n");
        return EXIT_FAILURE;
    }

    Elist alloced_frame;
    elist_init(&alloced_frame);

    bool flag = true;
    while (flag) {
        uint8_t order;
        int cmd = 0;
        size_t idx = 0;
        Frame* f = NULL;

        newline();
        printf("1. Allocate frame\n");
        printf("2. Free frame\n");
        printf("3. Show Allocated frames\n");
        printf("4. Show Buddy System state\n");
        printf("5. Exit\n");
        printf("Please select command : ");
        scanf("%d", &cmd);

        newline();
        print_separator();
        switch (cmd) {
            case 1:
                printf("Allocation frames\n");
                printf("Please input order to allocate : ");
                scanf("%" SCNu8, &order);
                f = buddy_alloc_frames(p, order);
                if (f == NULL) {
                    fprintf(stderr, "*Cannot Allocate Frame*\n");
                    break;
                }
                elist_insert_next(&alloced_frame, &f->list);
                printf("Allocation success!\n");
                break;
            case 2:
                printf("Free frames\n");
                printf("Please input idx to free : ");
                scanf("%zd", &idx);

                f = NULL;
                elist_foreach(Frame*, itr, &alloced_frame) {
                    if (idx == get_frame_idx(p, itr)) {
                        f = itr;
                    }
                }

                if (f == NULL) {
                    fprintf(stderr, "*Frame %zd is NOT allocated*\n", idx);
                    break;
                }

                buddy_free_frames(p, elist_get_frame(elist_remove(&f->list)));

                break;
            case 3:
                printf("Allocated frames");
                if (elist_is_empty(&alloced_frame)) {
                    printf(" is nothing\n");
                    break;
                }
                newline();

                elist_foreach(Frame*, itr, &alloced_frame) {
                    printf("  ");
                    print_frame_info(p, itr);
                }
                break;
            case 4:
                printf("Buddy System State\n");
                print_buddy_system(p);
                break;
            default:
                printf("Exit\n");
                flag = false;
                break;
        }
        print_separator();
    }

    buddy_destruct(p);

    return EXIT_SUCCESS;
}
