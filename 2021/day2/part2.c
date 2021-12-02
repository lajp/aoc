#include <stdio.h>

int main() {
    char dir[255];
    int num, f, a, d;
    f = d = a = 0;
    while(scanf("%s %d", dir, &num) != EOF) {
        switch(*dir) {
            case 'u':
                a -= num;
                break;
            case 'd':
                a += num;
                break;
            case 'f':
                f += num;
                d += a*num;
        }
    }
    printf("Answer: %d\n", f*d);
    return 0;
}
