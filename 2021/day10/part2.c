#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

const int vals[4] = {3, 57, 1197, 25137};


int cmpfunc(const void* a_, const void* b_) {
    long (*a)[2] = (long (*)[2]) a_;
    long (*b)[2] = (long (*)[2]) b_;

    long result;

    if ((*a)[0] - (*b)[0] != 0)
        result = (*a)[0] - (*b)[0];
    else
        result = (*a)[1] - (*b)[1];

    if (result < 0)
        return -1;
    else if (result > 0)
        return 1;
    else
        return 0;
}

int main() {
    long ans = 0;
    int in;
    bool nextline = false;
    char brackets[1000];
    int bpos = 0;
    long scores[1000];
    int scount = 0;
    while((in = getc_unlocked(stdin))) {
        if(in == EOF) {
            break;
        }
        if(in == '\n') {
            if(!nextline) {
                while(bpos > 0) {
                    ans *= 5;
                    ans += brackets[--bpos]+1;
                }
                scores[scount++] = ans;
                ans = 0;
            }
            nextline = false;
            bpos = 0;
            continue;
        }
        if(nextline) {
            continue;
        }
        switch(in) {
            case '(':
                brackets[bpos++] = 0;
                break;
            case '[':
                brackets[bpos++] = 1;
                break;
            case '{':
                brackets[bpos++] = 2;
                break;
            case '<':
                brackets[bpos++] = 3;
                break;
            case ')':
                if(brackets[--bpos] != 0) {
                    nextline = true;
                }
                break;
            case ']':
                if(brackets[--bpos] != 1) {
                    nextline = true;
                }
                break;
            case '}':
                if(brackets[--bpos] != 2) {
                    nextline = true;
                }
                break;
            case '>':
                if(brackets[--bpos] != 3) {
                    nextline = true;
                }
                break;
        }
    }
    qsort(scores, scount, sizeof(long), cmpfunc);
    printf("Answer: %ld\n", scores[(scount/2)]);
    return 0;
}
