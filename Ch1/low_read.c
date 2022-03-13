#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>

#define BUF_SIZE 100

void error_handling(char * msg) {
    perror(msg);
    exit(EXIT_FAILURE);
}

int main() {
    int fd = -1;
    ssize_t n_bytes = -1;
    char buf[BUF_SIZE];
    *buf = '\0';

    fd = open("data.txt", O_RDONLY);
    if (-1 == fd) {
        error_handling("open()");
    }
    printf("file descriptor: %d\n", fd);


    if (-1==(n_bytes= read(fd, buf, BUF_SIZE))) {
        error_handling("read()");
    }
    buf[n_bytes] = '\0';
    printf("file data: %s", buf);
    close(fd);

    return 0;
}