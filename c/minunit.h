/*
 * http://www.jera.com/techinfo/jtns/jtn002.html
 */

#ifndef _MIN_UNIT_H_
#define _MIN_UNIT_H_



static int minunit_test_counter;


#define MIN_UNIT_ASSERT(msg, expr) \
    do {                           \
        if ((expr) == 0) {         \
            return msg;            \
        }                          \
    } while (0)


#define MIN_UNIT_RUN_TEST(func_name)   \
    do {                               \
        char const *msg = func_name(); \
        minunit_test_counter++;        \
        if (msg != NULL) {             \
            return msg;                \
        }                              \
    } while (0)



#endif
