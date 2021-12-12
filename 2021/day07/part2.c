#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>
#include <time.h>

int main() {
    time_t start = clock();
    int minval, maxval, total, sum;
    total = maxval = 0;
    sum = minval = INT_MAX;
    int values[1000];
    char* line = malloc(4000*sizeof(char));
    int ll = 0;
    for(char in = getchar_unlocked(); in != EOF; in = getchar_unlocked()) {
        line[ll++] = in;
    }
    line[ll] = '\0';
    time_t start_noin = clock();
    const char* in;
    for(in = strtok(line, ","); in && *in; ++total) {
        int num = atoi(in);
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
    printf("%i\n", sum);
    time_t stop = clock();
    float time = (float)(stop-start)/(CLOCKS_PER_SEC)*1000;
    float time_noin = (float)(stop-start_noin)/(CLOCKS_PER_SEC)*1000;
    printf("Time: %fms, without input: %fms\n", time, time_noin);
    return 0;
}
