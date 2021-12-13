#include <stdio.h>
#include <stdbool.h>

int main() {
    bool map[2000][2000] = {0};
    int maxx = 0, maxy = 0;
    int x, y;
    while(scanf("%d,%d", &x, &y) > 0) {
        map[y][x] = 1;
        if(x > maxx) {
            maxx = x;
        }
        if(y > maxy) {
            maxy = y;
        }
    }
    char a;
    int f = 0;
    while(scanf("fold along %c=%d\n", &a, &y) != EOF) {
        if(a == 'x') {
            for(int i = 0; i <= maxy; ++i) {
                for(int j = y+1; j <= maxx; ++j) {
                    if(map[i][j]) {
                        map[i][j] = 0;
                        map[i][y-(j-y)] = 1;
                    }
                }
            }
            maxx = y;
        } else if(a == 'y') {
            for(int i = y+1; i <= maxy; ++i) {
                for(int j = 0; j <= maxx; ++j) {
                    if(map[i][j]) {
                        map[i][j] = 0;
                        map[y-(i-y)][j] = 1;
                    }
                }
            }
            maxy = y;
        }
    }
    for(int i = 0; i <= maxy; ++i) {
        for(int j = 0; j <= maxx; ++j) {
            if(map[i][j]) {
                printf("#");
            } else {
                printf(".");
            }
        }
        printf("\n");
    }
    return 0;
}
