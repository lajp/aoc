#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main() {
    static char in[255];
    scanf("%s", in);
    int len = strlen(in);
    int bits[len];
    char gamma[len];
    char epsilon[len];
    int total = 0;
    memset(bits, 0, len*sizeof(int));
    do {
        for(int i = 0; i < len; ++i) {
            bits[i] += (in[i] == '1');
        }
        total++;
    } while(scanf("%s", in) != EOF);
    for(int i = 0; i < len; ++i) {
        if(bits[i] > total/2) {
            gamma[i] = '1';
            epsilon[i] = '0';
        } else {
            gamma[i] = '0';
            epsilon[i] = '1';
        }
    }
    int e = strtol(epsilon, NULL, 2);
    int g = strtol(gamma, NULL, 2);
    printf("Answer: %d\n", e*g);
    return 0;
}
