#include <stdio.h>
#include <stdbool.h>

int rowc = 0;
int rowl = 0;

int find_adjacent(int i, int j, char (*map)[100], bool (*flashed)[100]) {
    int ans = 1;
    flashed[i][j] = true;
    map[i][j] = 0;
    if(i > 0) { // UP
        if(!flashed[i-1][j] && ++map[i-1][j] > 9) {
            ans += find_adjacent(i-1, j, map, flashed);
        }
        if(j > 0) { // UP-LEFT
            if(!flashed[i-1][j-1] && ++map[i-1][j-1] > 9) {
                ans += find_adjacent(i-1, j-1, map, flashed);
            }
        }
        if(j < rowl-1) { // UP-RIGHT
            if(!flashed[i-1][j+1] && ++map[i-1][j+1] > 9) {
                ans += find_adjacent(i-1, j+1, map, flashed);
            }
        }
    }
    if(i < rowc-1) { // DOWN
        if(!flashed[i+1][j] && ++map[i+1][j] > 9) {
            ans += find_adjacent(i+1, j, map, flashed);
        }
        if(j > 0) { // DOWN-LEFT
            if(!flashed[i+1][j-1] && ++map[i+1][j-1] > 9) {
                ans += find_adjacent(i+1, j-1, map, flashed);
            }
        }
        if(j < rowl-1) { // DOWN-RIGHT
            if(!flashed[i+1][j+1] && ++map[i+1][j+1] > 9) {
                ans += find_adjacent(i+1, j+1, map, flashed);
            }
        }
    }
    if(j > 0) { // LEFT
        if(!flashed[i][j-1] && ++map[i][j-1] > 9) {
            ans += find_adjacent(i, j-1, map, flashed);
        }
    }
    if(j < rowl-1) { // RIGHT
        if(!flashed[i][j+1] && ++map[i][j+1] > 9) {
            ans += find_adjacent(i, j+1, map, flashed);
        }
    }
    return ans;
}

int main() {
    char map[100][100];
    int ans = 0;
    int in;
    for(int i = 0; (in = getchar_unlocked()) != EOF; ++i) {
        if(in == '\n') {
            if(rowl == 0) {
                rowl = i;
            }
            rowc++;
            i = -1;
            continue;
        }
        map[rowc][i] = in-'0';
    }
    for(int g = 0; g < 100; ++g) {
        bool flashed[100][100] = {0};
        for(int i = 0; i < rowc; ++i) {
            for(int j = 0; j < rowl; ++j) {
                ++map[i][j];
            }
        }
        for(int i = 0; i < rowc; ++i) {
            for(int j = 0; j < rowl; ++j) {
                if(map[i][j] > 9) {
                    ans += find_adjacent(i, j, map, flashed);
                }
            }
        }
    }
    printf("Answer: %d\n", ans);
    return 0;
}
