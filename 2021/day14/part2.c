#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <stdbool.h>

typedef struct {
    char first;
    char second;
    long count;
} pair;

int find_pair(pair p, pair* ps, int pc) {
    for(int i = 0; i < pc; ++i) {
        if(p.first == ps[i].first && p.second == ps[i].second) {
            return i;
        }
    }
    return -1;
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
    pair* pairs = malloc(100*sizeof(pair));
    int paircount = 0;
    for(int ss = 0; ss < prevlen-1; ++ss) {
        pair p = { .first = ptr1[ss], .second = ptr1[ss+1], .count = 0 };
        int i = find_pair(p, pairs, paircount);
        if(i == -1) {
            i = paircount;
            pairs[paircount++] = p;
        }
        pairs[i].count++;
    }
    for(int i = 0; i < 40; ++i) {
        pair* newpairs = malloc(100*sizeof(pair));
        int newcount = 0;
        for(int j = 0; j < paircount; ++j) {
            char out = rulemap[pairs[j].first-'A'][pairs[j].second-'A'];
            if(out == -1) {
                continue;
            }
            letters[out]+=pairs[j].count;
            out+='A';
            pair p1 = { .first = pairs[j].first, .second = out, .count = 0 };
            pair p2 = { .first = out, .second = pairs[j].second, .count = 0 };
            int ind = find_pair(p1, newpairs, newcount);
            if(ind == -1) {
                ind = newcount;
                newpairs[newcount++] = p1;
            }
            newpairs[ind].count+=pairs[j].count;
            ind = find_pair(p2, newpairs, newcount);
            if(ind == -1) {
                ind = newcount;
                newpairs[newcount++] = p2;
            }
            newpairs[ind].count+=pairs[j].count;
        }
        paircount = newcount;
        free(pairs);
        pairs = newpairs;
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
