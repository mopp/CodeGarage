#include <stdio.h>
#include <unistd.h>
#include <string.h>


int main(int argc, char const* const argv[]) {
    static char* const msg[BUFSIZ];

    for (size_t i = 0; i < BUFSIZ / 4; i++) {
        memcpy((void*)msg + i * 4, "yes\n", 4);
    }

    for (;;) {
        write(STDOUT_FILENO, msg, BUFSIZ);
    }

    return 0;
}
