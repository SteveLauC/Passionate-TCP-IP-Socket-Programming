#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>

void error_handling(char * msg) {
    perror(msg);
    exit(EXIT_FAILURE);
}

int main() {
    int fd;
    char buf[] = "let's go\n";

    // mode argument must be specified if O_CREAT is used
    fd = open("data.txt", O_WRONLY|O_CREAT|O_TRUNC, 0644);
    if (-1 == fd) {
        error_handling("open()");
    }
    printf("file descriptor: %d\n", fd);

    if (-1 == write(fd, buf, strlen(buf))) {
        error_handling("write()");
    }

    close(fd);
    return 0;
}