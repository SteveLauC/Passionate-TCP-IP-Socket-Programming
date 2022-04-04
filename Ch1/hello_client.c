#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/socket.h>

void error_handling(char * msg) {
    fputs(msg, stderr);
    perror(NULL);
    fputc('\n', stderr);
    exit(EXIT_FAILURE);
}


int main(int ac, char * av[]) {
    int sock;
    struct sockaddr_in serv_addr;
    char message[30]; *message='\0';
    int str_len;

    if (ac!=3) {
        fprintf(stderr, "Usage: %s <IP> <port>\n", av[0]);
        exit(EXIT_FAILURE);
    }

    sock = socket(PF_INET, SOCK_STREAM, 0);
    if (-1 == sock) {
        error_handling("socket() error");
    }

    memset(&serv_addr, 0, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_addr.s_addr = inet_addr(av[1]);
    serv_addr.sin_port = htons(atoi(av[2]));

    if (-1 == connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr))) {
        error_handling("connect() error");
    }

    str_len = read(sock, message, sizeof(message)-1);
    if (-1 == str_len) {
        error_handling("read() error");
    }
    message[str_len] = '\0';

    printf("message from server: %s\n", message);
    close(sock);
    return 0;
}
