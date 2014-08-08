#############################################################
# @file Makefile
# @brief This is used for testing.
# @author mopp
# @version 0.1
# @date 2014-08-07
#############################################################

RM 			:= rm -rf
CC			:= clang
MAKE 		:= make
MAKEFILE 	:= Makefile


.PHONY: test
test: $(MAKEFILE)
	$(MAKE) dlist


.PHONY: dlist
dlist: $(MAKEFILE) ../dlist.c ./test_dlist.c
	$(CC) ../$@.c ./test_$@.c -o $@.o
	@echo ''
	./$@.o
	@echo ''


.PHONY: clean
clean:
	$(RM) *.o
