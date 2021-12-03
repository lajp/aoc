#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

bool gammabit(char** nums, bool* elimed, int total, int index) {
    int s = 0;
    int elim = 0;
    for(int j = 0; j < total; ++j) {
        if(!elimed[j]) {
            s += (nums[j][index] == '1');
        } else {
            elim++;
        }
    }
    return (s >= total-elim-s);
}

int main() {
    static char in[255];
    scanf("%s", in);
    int len = strlen(in);
    int total = 0;
    char** nums = malloc(1000*sizeof(char*)); // This is cheating
    do {
        nums[total] = malloc(len*sizeof(char));
        strcpy(nums[total], in);
        total++;
    } while(scanf("%s", in) != EOF);
    int c, o, elimg, elime;
    elimg = elime = 0;
    bool elimedg[total];
    bool elimede[total];
    memset(elimedg, 0, total*sizeof(bool));
    memset(elimede, 0, total*sizeof(bool));
    for(int i = 0; i < len; ++i) {
        bool gamma = gammabit(nums, elimedg, total, i);
        bool epsilon = !gammabit(nums, elimede, total, i);
        for(int j = 0; j < total; ++j) {
            if(!elimedg[j] && (nums[j][i]-'0' != gamma)) {
                elimg++;
                elimedg[j] = true;
            }
            if(!elimede[j] && (nums[j][i]-'0' != epsilon)) {
                elime++;
                elimede[j] = true;
            }
        }
        if(elimg == total-1) {
            for(int j = 0; j < total; ++j) {
                if(elimedg[j] == 0) {
                    o = strtol(nums[j], NULL, 2);
                }
            }
        }
        if(elime == total-1) {
            for(int j = 0; j < total; ++j) {
                if(elimede[j] == 0) {
                    c = strtol(nums[j], NULL, 2);
                }
            }
        }
    }
    printf("Answer: %d\n", o*c);
    return 0;
}
