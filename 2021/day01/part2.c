#include <stdio.h>

int main() {
    static int sums[4];
    int pos, r, ans;
    ans = pos = 0;
    while(scanf("%d", &r) != EOF) {
        switch(pos%4) {
            case 0:
                sums[0]+=r;
                sums[2]+=r;
                sums[3]+=r;
                ans += (pos > 2) && (sums[2] > sums[1]);
                sums[1] = 0;
                break;
            case 1:
                sums[0]+=r;
                sums[1]+=r;
                sums[3]+=r;
                ans += (pos > 2) && (sums[3] > sums[2]);
                sums[2] = 0;
                break;
            case 2:
                sums[0]+=r;
                sums[1]+=r;
                sums[2]+=r;
                ans += (pos > 2) && (sums[0] > sums[3]);
                sums[3] = 0;
                break;
            case 3:
                sums[1]+=r;
                sums[2]+=r;
                sums[3]+=r;
                ans += sums[1] > sums[0];
                sums[0] = 0;
                break;
        }
        ++pos;
    }
    printf("Answer: %d\n", ans);
    return 0;
}
