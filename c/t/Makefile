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
	$(MAKE) aqueue
	$(MAKE) lqueue
	$(MAKE) memory_dump


.PHONY: dlist
dlist: $(MAKEFILE) ../dlist.c ./test_dlist.c
	$(CC) ../$@.c ./test_$@.c -o $@.o
	@echo ''
	./$@.o
	@echo ''

.PHONY: aqueue
aqueue: $(MAKEFILE) ../aqueue.c ./test_aqueue.c
	$(CC) ../$@.c ./test_$@.c -o $@.o
	@echo ''
	./$@.o
	@echo ''

.PHONY: lqueue
lqueue: $(MAKEFILE) ../dlist.c ../lqueue.c ./test_lqueue.c
	$(CC) ../dlist.c ../$@.c ./test_$@.c -o $@.o
	@echo ''
	./$@.o
	@echo ''

.PHONY: memory_dump
memory_dump: $(MAKEFILE) ../memory_dump.c  ./test_memory_dump.c
	$(CC) ../$@.c ./test_$@.c -o $@.o
	@echo ''
	./$@.o
	@echo ''

.PHONY: clean
clean:
	$(RM) *.o
