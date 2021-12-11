#include <stdio.h>
#include <stdbool.h>

int rowlen = 0;
int rowc = 0;

int main() {
    int ans = 0;
    char map[100][100] = {0};
    int k = 0;
    char n;
    while(scanf("%c", &n) != EOF) {
        if(n == '\n') {
            if(rowlen == 0) {
                rowlen = k;
            }
            rowc++;
            k = 0;
        } else {
            map[rowc][k++] = n-'0';
        }
    }
    for(int i = 0; i < rowc; ++i) {
        for(int j = 0; j < rowlen; ++j) {
            bool low = false;
            if(j > 0) {
                low = (map[i][j] < map[i][j-1]);
                if(!low) {
                    continue;
                }
            }
            if(j < rowlen-1) {
                low = (map[i][j] < map[i][j+1]);
                if(!low) {
                    continue;
                }
            }
            if(i > 0) {
                low = (map[i][j] < map[i-1][j]);
                if(!low) {
                    continue;
                }
            }
            if(i < rowc-1) {
                low = (map[i][j] < map[i+1][j]);
                if(!low) {
                    continue;
                }
            }
            ans += map[i][j]+1;
        }
    }
    printf("Answer: %d\n", ans);
    return 0;
}
