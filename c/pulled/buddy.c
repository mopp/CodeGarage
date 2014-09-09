/*
 * BUDDY SYSTEM CODE
 * http://www.sourcecodesworld.com/source/show.asp?ScriptID=1077
 */
#include <stdio.h>


static void segmentalloc(int, int);
static void makedivided(int);
static void makefree(int);
static void printing(int, int);
static int place(int);
static int power_2(int);
static void frame(int);
static void frame_echo(char const* const);
static int mod2(int);
static int div2(int);

static int tree[2050];
static char const* frame_str = "================================================================================";



int main(void) {
    int total_size, choice, request_size;

    frame_echo("\tB U D D Y   S Y S T E M  R E Q U I R E M E N T S");

    printf("*  Enter the Size of the memory  :  ");

    scanf("%d", &total_size);

    while (1) {
        frame_echo("\tB U D D Y   S Y S T E M ");
        puts(" *  1)\tLocate the process into the Memory");
        puts(" *  2)\tRemove the process from Memory");
        puts(" *  3)\tTree structure for Memory allocation Map");
        puts(" *  4)\tExit");
        printf(" *  Enter your choice : ");

        scanf(" %d", &choice);

        switch (choice) {
            case 1:
                frame_echo("\tM E M O R Y   A L L O C A T I O N ");
                printf(" *  Enter the Process size  : ");

                scanf("%d", &request_size);

                segmentalloc(total_size, request_size);

                break;
            case 2:
                frame_echo("\tM E M O R Y   D E A L L O C A T I O N ");
                printf(" *  Enter the process size  :  ");

                scanf("%d", &request_size);

                makefree(request_size);

                break;
            case 3:
                frame_echo("\tM E M O R Y   A L L O C A T I O N   M A P ");

                printing(total_size, 0);

                putchar('\n');

                break;
            default:
                return 0;
        }
    }
}


static inline void frame(int is_newline) {
    printf("%s%c", frame_str, (is_newline != 0 ? '\n' : ' '));
}


static inline void frame_echo(char const* const str) {
    frame(1);
    puts(str);
    frame(1);
}


static inline void segmentalloc(int total_size, int request) {
    int i, flevel = 0, size = total_size;

    if (total_size < request) {
        puts(" *  Result:");
        puts(" *  The system don't have enough free memory");
        puts(" *  Suggession  :  Go for VIRTUAL MEMORY");

        return;
    }

    while (1) {
        if (request <= size && div2(size) < request) {
            break;
        } else {
            size = div2(size);
            flevel++;
        }
    }

    for (i = power_2(flevel) - 1; i <= (power_2(flevel + 1) - 2); i++) {
        if (tree[i] == 0 && place(i)) {
            tree[i] = request;
            makedivided(i);
            puts(" *  Result : Successful Allocation");
            break;
        }
    }

    if (i == power_2(flevel + 1) - 1) {
        puts(" *  Result :");
        puts(" *  The system don't have enough free memory");
        puts(" *  Suggession : Go for VIRTUAL Memory Mode");
    }
}


static inline void makedivided(int node) {
    while (node != 0) {
        node = (mod2(node) == 0) ? div2(node - 1) : div2(node);
        tree[node] = 1;
    }
}


static inline int place(int node) {
    while (node != 0) {
        node = (mod2(node) == 0) ? div2(node - 1) : div2(node);
        if (1 < tree[node]) {
            return 0;
        }
    }

    return 1;
}


static inline void makefree(int request) {
    int node = 0;

    while (tree[node] != request) {
        node++;
    }

    tree[node] = 0;
    while (node != 0) {
        if ((tree[(mod2(node) == 0) ? node - 1 : node + 1] == 0) && tree[node] == 0) {
            tree[(mod2(node) == 0) ? div2(node - 1) : div2(node)] = 0;
            node = (mod2(node) == 0) ? div2(node - 1) : div2(node);
        } else {
            break;
        }
    }
}


static inline int power_2(int n) {
    return (n == 0) ? (1) : (2 << (n - 1));
}


static inline void printing(int total_size, int node) {
    int permission = 0, llimit, ulimit, tab;

    if (node == 0) {
        permission = 1;
    } else if (mod2(node) == 0) {
        permission = (tree[div2(node - 1)] == 1) ? 1 : 0;
    } else {
        permission = (tree[div2(node)] == 1) ? 1 : 0;
    }

    if (permission) {
        llimit = ulimit = tab = 0;

        while (!(llimit <= node && node <= ulimit)) {
            tab++;
            putchar('\t');
            llimit = ulimit + 1;
            ulimit = 2 * ulimit + 2;
        }

        printf(" %d ", total_size / power_2(tab));

        if (1 < tree[node]) {
            printf("---> Allocated %d\n", tree[node]);
        } else if (tree[node] == 1) {
            printf("---> Divided\n");
        } else {
            printf("---> Free\n");
        }

        printing(total_size, 2 * node + 1);
        printing(total_size, 2 * node + 2);
    }
}


static inline int mod2(int x) {
    return x & 0x1;
}


static inline int div2(int x) {
    return x >> 0x1;
}
