/**
 * @file macro.h
 * @brief maybe useful macros.
 * @author mopp
 * @version 0.1
 * @date 2014-08-07
 */


#ifndef _MACRO_H_
#define _MACRO_H_



#define QUOTE(x) #x
#define TO_STR(x) QUOTE(x)
#define CURRENT_LINE_STR TO_STR(__LINE__)
#define CURRENT_FUNC_STR TO_STR(__FUNCTION__)
#define HERE_STRING (__FILE__ " " CURRENT_LINE_STR ":")



#endif
