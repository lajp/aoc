#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>

int main() {
    int minval, maxval, total, sum;
    total = maxval = 0;
    sum = minval = INT_MAX;
    int values[1000];
    char* line = malloc(128*sizeof(char));
    const char* in;
    int ll = 0;
    for(char in = getchar_unlocked(); in != EOF; in = getchar_unlocked()) {
        line[ll++] = in;
    }
    line[ll] = '\0';
    for(in = strtok(line, ","); in && *in; ++total) {
        int num = strtol(in, NULL, 10);
        values[total] = num;
        if(num < minval) {
            minval = num;
        } else if(num > maxval) {
            maxval = num;
        }
        in = strtok(NULL, ",");
    }
    for(int i = minval; i < maxval; ++i) {
        int fsum = 0;
        for(int j = 0; j < total; ++j) {
            int n = abs(i-values[j]);
            fsum += (n*(n+1))/2;
        }
        if(fsum < sum) {
            sum = fsum;
        }
    }
    printf("Answer: %d\n", sum);
    return 0;
}
