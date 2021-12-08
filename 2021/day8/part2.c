#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

void permute(char* s, int i, int len, char** out, int* n) {
    if(i == len) {
        out[*n] = malloc((len+1)*sizeof(char)); // '\0'
        strcpy(out[*n], s);
        *n = *n + 1;
    } else {
        for(int j = i; j < len; ++j) {
            int temp;
            temp = s[i];
            s[i] = s[j];
            s[j] = temp;
            permute(s, i+1, len, out, n);
            temp = s[i];
            s[i] = s[j];
            s[j] = temp;
        }
    }
}

bool numeq(char* s1, char* s2) {
    int l1 = strlen(s1);
    int l2 = strlen(s2);
    if(l1 != l2) {
        return false;
    }
    bool t1[7] = {0};
    bool t2[7] = {0};
    for(int i = 0; i < l1; ++i) {
        t1[s1[i]-'a']++;
        t2[s2[i]-'a']++;
    }
    for(int i = 0; i < l1; ++i) {
        if(t1[i] != t2[i]) {
            return false;
        }
    }
    return true;
}

int factorial(int n) {
    if(n == 1) {
        return 1;
    }
    return n*factorial(n-1);
}

char* digits[] = {
    "abcefg",
    "cf",
    "acdeg",
    "acdfg",
    "bcdf",
    "abdfg",
    "abdefg",
    "acf",
    "abcdefg",
    "abcdfg"
};

int main() {
    char init[8] = "abcdefg";
    int permamount = 0;
    char** permutations = malloc(factorial(7)*sizeof(char*));
    permute(init, 0, strlen(init), permutations, &permamount);
    size_t size = 1000;
    char* line = NULL;
    int ans = 0;
    while(getline(&line, &size, stdin) != EOF) {
        const char* in;
        char nums[100][8];
        int namount = 0;
        for(in = strtok(line, " "); in && *in; in = strtok(NULL, " ")) {
            strcpy(nums[namount++], in);
        }
        for(int i = 0; i < permamount; ++i) {
            bool failed = false;
            int j;
            for(j = 0; j < namount; ++j) {
                if(strcmp(nums[j], "|") == 0) {
                    break;
                }
                int templen = 0;
                char tempstr[8] = {0};
                for(int k = 0; k < 7; ++k) {
                    if(strchr(nums[j], init[k]) != NULL) {
                        tempstr[templen++] = permutations[i][k];
                    }
                }
                tempstr[templen] = '\0';
                bool found = false;
                for(int k = 0; k < 10; ++k) {
                    found += numeq(digits[k], tempstr);
                    if(found) {
                        found = true;
                        break;
                    }
                }
                if(!found) {
                    failed = true;
                    break;
                }
            }
            if(!failed) {
                char numstr[100] = {0};
                int numlen = 0;
                for(j++; j < namount; ++j) {
                    int templen = 0;
                    char tempstr[8] = {0};
                    for(int k = 0; k < 7; ++k) {
                        if(strchr(nums[j], init[k]) != NULL) {
                            tempstr[templen++] = permutations[i][k];
                        }
                    }
                    tempstr[templen] = '\0';
                    for(int k = 0; k < 10; ++k) {
                        if(numeq(digits[k], tempstr)) {
                            numstr[numlen++] = k+'0';
                            break;
                        }
                    }
                }
                numstr[numlen] = '\0';
                ans += strtol(numstr, NULL, 10);
                break;
            }
        }
    }
    printf("Answer: %d\n", ans);
    return 0;
}
