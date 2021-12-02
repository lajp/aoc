#include <stdio.h>

int main() {
    char dir[255];
    int num, f, d;
    f = d = 0;
    while(scanf("%s %d", dir, &num) != EOF) {
        switch(*dir) {
            case 'u':
                d -= num;
                break;
            case 'd':
                d += num;
                break;
            case 'f':
                f += num;
        }
    }
    printf("Answer: %d\n", f*d);
    return 0;
}
