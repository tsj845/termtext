#include <stdio.h>

int main(int argc, char **argv) {
    for (int i = 0; i < argc; i ++) {
        printf("%s\r\n", argv[i]);
    }
    return 0;
}