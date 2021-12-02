#include <stdio.h>
#include <string.h>

int main() {
    char dir[255];
    int num, f, a, d;
    f = d = a = 0;
    while(scanf("%s %d", &dir[0], &num) != EOF) {
        if(strcmp(dir, "up") == 0) {
            a -= num;
        } else if(strcmp(dir, "down") == 0) {
            a += num;
        } else if(strcmp(dir, "forward") == 0) {
            f += num;
            d += a*num;
        }
    }
    printf("Answer: %d\n", f*d);
    return 0;
}
