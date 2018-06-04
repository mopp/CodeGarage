075: 075_lib.nas 075_main.nas
	nasm -f elf64 -o 075_lib.o 075_lib.nas
	nasm -f elf64 -o 075_main.o 075_main.nas
	ld -o $@ 075_main.o 075_lib.o

.PHONY: clean
clean:
	rm -f 075_lib.o 075_main.o 075
