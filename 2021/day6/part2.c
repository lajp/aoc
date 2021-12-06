#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    long fishamount = 0;
    long* fstates = malloc(9*sizeof(long));
    memset(fstates, 0, 9*sizeof(long));
    size_t s = 255;
    char* line = NULL;
    getline(&line, &s, stdin);
    const char* in;
    for(in = strtok(line, ","); in && *in; ++fishamount) {
        fstates[strtol(in, NULL, 10)]++;
        in = strtok(NULL, ",");
    }
    free(line);
    for(int i = 0; i < 256; ++i) {
        long* nfstates = malloc(9*sizeof(long));
        memset(nfstates, 0, 9*sizeof(long));
        for(int j = 0; j < 9; ++j) {
            if(j == 0) {
                nfstates[8] = fstates[0];
                nfstates[6] += fstates[0];
                fishamount += fstates[0];
            } else {
                nfstates[j-1] += fstates[j];
            }
        }
        long* old = fstates;
        fstates = nfstates;
        free(old);
    }
    printf("Answer: %ld\n", fishamount);
}
