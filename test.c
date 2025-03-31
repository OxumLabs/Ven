#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    char* name = malloc(256);
    fgets(name, 256, stdin);
    size_t len = strlen(name);
    if(len > 0 && name[len-1] == '\n') name[len-1] = '\0';
    fprintf(stdout, "%s , nice to meet u!", name);
    return 0;
}
