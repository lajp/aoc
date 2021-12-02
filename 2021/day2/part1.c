#include <stdio.h>
#include <string.h>

int main() {
    char dir[255];
    int num, f, d;
    f = d = 0;
    while(scanf("%s %d", &dir[0], &num) != EOF) {
        if(strcmp(dir, "up") == 0) {
            d -= num;
        } else if(strcmp(dir, "down") == 0) {
            d += num;
        } else if(strcmp(dir, "forward") == 0) {
            f += num;
        }
    }
    printf("Answer: %d\n", f*d);
    return 0;
}
