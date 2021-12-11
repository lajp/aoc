#include <stdio.h>
#include <stdbool.h>

int rowlen = 0;
int rowc = 0;

int basin(int i, int j, char (*map)[100], bool (*visited)[100]) {
    int size = 1;
    visited[i][j] = true;
    if(j > 0 && map[i][j-1] != 9 && !visited[i][j-1]) {
        size += basin(i, j-1, map, visited);
    }
    if(j < rowlen-1 && map[i][j+1] != 9 && !visited[i][j+1]) {
        size += basin(i, j+1, map, visited);
    }
    if(i > 0 && map[i-1][j] != 9 && !visited[i-1][j]) {
        size += basin(i-1, j, map, visited);
    }
    if(i < rowc-1 && map[i+1][j] != 9 && !visited[i+1][j]) {
        size += basin(i+1, j, map, visited);
    }
    return size;
}

int main() {
    char map[100][100] = {0};
    int basins[100];
    int bamount = 0;
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
            bool visited[100][100] = {false};
            basins[bamount++] = basin(i, j, map, visited);
        }
    }
    int a1, a2, a3;
    a1 = a2 = a3 = 0;
    for(int i = 0; i < bamount; ++i) {
        if(basins[i] > a3) {
            if(basins[i] > a2) {
                if(basins[i] > a1) {
                    a3 = a2;
                    a2 = a1;
                    a1 = basins[i];
                } else {
                    a3 = a2;
                    a2 = basins[i];
                }
            } else {
                a3 = basins[i];
            }
        }
    }
    printf("%d\n", a1*a2*a3);
    return 0;
}
