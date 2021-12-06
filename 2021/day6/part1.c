#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * I find this solution to be so beautiful
 * I'm gonna keep it even though my solution
 * in part2 is objectively better.
 *
 * But tbf part1-me had no idea what was coming up
 * in part2.. It could've been something for which
 * this solution would've been an excellent basis
*/

typedef struct fish {
    char timer;
    struct fish* next;
} fish;

int main() {
    int fishamount = 0;
    fish* fishlist = NULL;
    fish* fishtail = NULL;
    fish* curfish = NULL;
    char* line = NULL;
    size_t s = 255;
    getline(&line, &s, stdin);
    const char* in;
    fish* f;
    for(in = strtok(line, ","); in && *in; ++fishamount) {
        f = malloc(sizeof(fish));
        f->next = NULL;
        f->timer = strtol(in, NULL, 10);
        if(!fishlist) {
            fishlist = f;
            curfish = f;
        }
        else {
            curfish->next = f;
            curfish = f;
        }
        in = strtok(NULL, ",");
    }
    fishtail = f;
    free(line);
    for(int i = 0; i < 80; ++i) {
        int toadd = 0;
        for(fish* f = fishlist; f; f = f->next) {
            if(f->timer-- == 0) {
                ++toadd;
                f->timer = 6;
            }
        }
        for(int j = 0; j < toadd; ++j) {
            fish* nf = malloc(sizeof(fish));
            nf->timer = 8;
            nf->next = NULL;
            fishtail->next = nf;
            fishtail = nf;
            ++fishamount;
        }
    }
    printf("Answer: %d\n", fishamount);
}
