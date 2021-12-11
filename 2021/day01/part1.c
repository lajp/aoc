#include <stdio.h>

int main() {
    int cur, prev;
    int ans = 0;
    scanf("%d", &prev);
    while(scanf("%d", &cur) != EOF) {
        ans += cur > prev;
        prev = cur;
    }
    printf("Answer: %d\n", ans);
    return 0;
}
