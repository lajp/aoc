#include <stdio.h>
#include <string.h>
#include <stdbool.h>

int main() {
    int lookfor[] = { 2, 4, 3, 7 };
    char* line = NULL;
    size_t size = 1000;
    const char* in;
    int ans = 0;
    while(getline(&line, &size, stdin) != EOF) {
        bool after = false;
        for(in = strtok(line, " "); in && *in; in = strtok(NULL, " ")) {
            if(*in == '|') {
                after = true;
                continue;
            }
            if(after) {
                int len = strlen(in);
                if(in[len-1] == '\n') { len--; }
                for(int i = 0; i < sizeof(lookfor)/sizeof(int); ++i) {
                    if(len == lookfor[i]) {
                        ans++;
                        break;
                    }
                }
            }
        }
        line = NULL;
    }
    printf("Answer: %d\n", ans);
    return 0;
}
