#include <stdio.h>
#include <stdlib.h>

int main(int argc, char** argv){
    FILE* f;
    if ((f = fdopen(1, "w")) == NULL) {
        perror("fdopen failed");
        exit(1);
    }
    char buf[5] = {'t','e','s','t','\n'};
    printf("bytes written: %lu\n", fwrite(buf, 1, 5, f));
    fclose(f);
    return 0;
}