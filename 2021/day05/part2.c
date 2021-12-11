#include <stdio.h>

#define MAXX(l) (l.sx > l.ex ? l.sx : l.ex)
#define MINX(l) (l.sx > l.ex ? l.ex : l.sx)
#define MAXY(l) (l.sy > l.ey ? l.sy : l.ey)
#define MINY(l) (l.sy > l.ey ? l.ey : l.sy)

typedef struct {
    int sx;
    int sy;
    int ex;
    int ey;
} line;

int main() {
    line lines[500];
    int x1, y1, x2, y2;
    int linecount = 0;
    while(scanf("%d,%d -> %d,%d", &x1, &y1, &x2, &y2) != EOF) {
        lines[linecount].sx = x1;
        lines[linecount].sy = y1;
        lines[linecount].ex = x2;
        lines[linecount].ey = y2;
        linecount++;
    }
    int ans = 0;
    static char map[1000][1000];
    for(int i = 0; i < linecount; ++i) {
        if(lines[i].sx == lines[i].ex) {
            for(int y = MINY(lines[i]); y <= MAXY(lines[i]); ++y) {
                if(++map[lines[i].sx][y] == 2) {
                    ans++;
                }
            }
        } else if(lines[i].sy == lines[i].ey) {
            for(int x = MINX(lines[i]); x <= MAXX(lines[i]); ++x) {
                if(++map[x][lines[i].sy] == 2) {
                    ans++;
                }
            }
        } else {
            int k = (lines[i].ex-lines[i].sx)/(lines[i].ey-lines[i].sy);
            int c = (k*(-lines[i].sx))+lines[i].sy;
            for(int x = MINX(lines[i]); x <= MAXX(lines[i]); ++x) {
                int y = (k*x)+c;
                if(++map[x][y] == 2) {
                    ans++;
                }
            }
        }
    }
    printf("Answer: %d\n", ans);
    return 0;
}
