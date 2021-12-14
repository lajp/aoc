#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

void expand_pair(char first, char second, int depth, char (*rules)[100], long long* letters) {
    if(depth >= 10) { return; }
    char to = rules[first-'A'][second-'A'];
    if(to != -1) {
        letters[to]++;
        expand_pair(first, to+'A', depth+1, rules, letters);
        expand_pair(to+'A', second, depth+1, rules, letters);
    }
    return;
}

int main() {
    char* ptr1 = NULL;
    size_t s = 1280;
    getline(&ptr1, &s, stdin);
    scanf("\n");
    char ra, rb, rr;
    char rulemap[100][100] = {-1};
    while(scanf("%c%c -> %c\n", &ra, &rb, &rr) != EOF) {
        rulemap[ra-'A'][rb-'A'] = rr-'A';
    }
    int prevlen = strlen(ptr1);
    ptr1[--prevlen] = '\0';
    long long letters[28] = {0};
    for(int i = 0; i < prevlen; ++i) {
        letters[ptr1[i]-'A']++;
    }
    for(int ss = 0; ss < prevlen-1; ++ss) {
        expand_pair(ptr1[ss], ptr1[ss+1], 0, rulemap, letters);
    }
    long long max = 0, min = LONG_MAX;
    for(int i = 0; i < 28; ++i) {
        if(letters[i] > max) {
            max = letters[i];
        }
        if(letters[i] < min && letters[i] > 0) {
            min = letters[i];
        }
    }
    printf("Answer: %lld\n", max-min);
    return 0;
}
