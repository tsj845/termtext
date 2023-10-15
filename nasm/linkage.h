// #include <stdio.h>
// #include "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Versions/Current/Headers/sys/errno.h"
#include <sys/errno.h>

extern int errno;
int _EBADF;

int _init_vars(){
    _EBADF = EBADF;
    return 0;
}
int _get_errno(){
    return errno;
}


// int main(int argc, char** argv){
//     printf("X: %d\n", EBADF);
//     return 0;
// }