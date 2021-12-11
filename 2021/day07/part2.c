#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>

int main() {
    int minval, maxval, total, sum;
    total = maxval = 0;
    sum = minval = INT_MAX;
    int values[1000];
    char* line = NULL;
    size_t s = 128;
    getline(&line, &s, stdin);
    const char* in;
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
    free(line);
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
