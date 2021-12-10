#include <stdio.h>
#include <stdbool.h>

const int vals[4] = {3, 57, 1197, 25137};

int main() {
    int ans = 0;
    int in;
    bool nextline = false;
    char brackets[1000];
    int bpos = 0;
    while((in = getc_unlocked(stdin))) {
        if(in == EOF) {
            break;
        }
        if(in == '\n') {
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
                    ans += vals[0];
                }
                break;
            case ']':
                if(brackets[--bpos] != 1) {
                    nextline = true;
                    ans += vals[1];
                }
                break;
            case '}':
                if(brackets[--bpos] != 2) {
                    nextline = true;
                    ans += vals[2];
                }
                break;
            case '>':
                if(brackets[--bpos] != 3) {
                    nextline = true;
                    ans += vals[3];
                }
                break;
        }
    }
    printf("Answer: %d\n", ans);
    return 0;
}
