#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>


void error_handing(char *msg) {
    perror(NULL);
    fputs(msg, stderr);
    fputc('\n', stderr);
    exit(EXIT_FAILURE);
}

int main(int ac, char *av[]) {
    int serv_sock;
    int clnt_sock;

    struct sockaddr_in serv_addr;
    struct sockaddr_in clnt_addr;
    socklen_t clnt_addr_size;

    char message[] = "Hello World!";

    if (ac != 2) {
        fprintf(stderr, "Usage: %s <port>\n", av[0]);
        exit(EXIT_FAILURE);
    }

    // create socket 
    // `PF_INET` means IPV4 internet protocols
    serv_sock = socket(PF_INET, SOCK_STREAM, 0);
    if (-1 == serv_sock) {
        error_handing("socket() error");
    }


    // prepare the ip address and port number
    memset(&serv_addr, 0, sizeof(serv_addr));
    // Address to accept any incoming messages.  */
    // #define	INADDR_ANY		((in_addr_t) 0x00000000)
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    serv_addr.sin_port = htons(atoi(av[1]));

    // bind the address to the socket
    if (-1 == bind(serv_sock, (struct sockaddr * )&serv_addr, sizeof(serv_addr))) {
        error_handing("bind() error");
    }

    // mark the socket avtive
    if (-1 == listen(serv_sock, 5)) {
        error_handing("listen() error");
    }

    clnt_addr_size = sizeof(clnt_addr);
    // The addrlen argument is a value-result argument: 
    // the caller must initialize it to contain the size 
    // (in bytes) of the structure pointed to by addr; 
    // on return it will contain the actual size of the peer address.
    clnt_sock = accept(serv_sock, (struct sockaddr *)&clnt_addr, &clnt_addr_size);
    if (-1 == clnt_sock) {
        error_handing("accept() error");
    }
    
    write(clnt_sock, message, sizeof(message));
    close(serv_sock);
    close(clnt_sock);
    return 0;
}