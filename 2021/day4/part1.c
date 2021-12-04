#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

typedef struct {
    int b[25];
    int matched;
} board;

const board wins[] = {
    {1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0},
    {0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0},
    {0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0},
    {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0},
    {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1},
    {1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0},
    {0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0},
    {0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0},
    {0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0},
    {0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1},
    /*{1,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,1},
    {0,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,0},
    FTW AMERICANS??!*/
};

bool checkwin(board* b) {
    for(int w = 0; w < sizeof(wins)/sizeof(board); ++w) {
        int a = 0;
        for(int i = 0; i < 25; ++i) {
            a += (wins[w].b[i] == 1) && (b->b[i] < 0);
        }
        if(a == 5) {
            return true;
        }
    }
    return false;
}

void addnum(board* boards, int btotal, int num) {
    for(int i = 0; i < btotal; ++i) {
        for(int j = 0; j < 25; ++j) {
            if(boards[i].b[j] == num) {
                boards[i].b[j] = -1;
                boards[i].matched++;
            }
        }
    }
}

int main() {
    int nums[255];
    int ntotal = 0;
    char* line = NULL;
    size_t s = 128;
    getline(&line, &s, stdin);
    const char* in;
    for(in = strtok(line, ","); in && *in; ++ntotal) {
        nums[ntotal] = strtol(in, NULL, 10);
        in = strtok(NULL, ",");
    }
    free(line);
    board* boards = malloc(1000*sizeof(board));
    int btotal = 0;
    for(int n; scanf("%d", &n) != EOF; ++btotal)  {
        boards[btotal].b[0] = n;
        for(int i = 1; i < 25; ++i) {
            scanf("%d", &boards[btotal].b[i]);
        }
        boards[btotal].matched = 0;
    }
    int ans = 0;
    for(int i = 0; i < ntotal; ++i) {
        bool done = false;
        addnum(boards, btotal, nums[i]);
        if(i > 5) {
            for(int b = 0; b < btotal; ++b) {
                if(boards[b].matched >= 5) {
                    if(checkwin(&boards[b])) {
                        for(int j = 0; j < 25; ++j) {
                            if(boards[b].b[j] > 0) {
                                ans+=boards[b].b[j];
                            }
                        }
                        ans *= nums[i];
                        done = true;
                        break;
                    }
                }
            }
            if(done) {
                break;
            }
        }
    }
    printf("Answer: %d\n", ans);
    return 0;
}
