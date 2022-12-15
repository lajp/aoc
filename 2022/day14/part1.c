#include <stdio.h>
#include <string.h>
#include <limits.h>

#define MAX(x, y) (x > y ? x : y)
#define MIN(x, y) (x > y ? y : x)

typedef struct {
    int scoords[2];
    int ecoords[2];
} line;

typedef struct {
    int p[2];
} sand;

char
in_line(int p[2], line l)
{
    if (l.scoords[0] == l.ecoords[0] && l.scoords[0] == p[0]) {
        if (p[1] <= MAX(l.scoords[1], l.ecoords[1]) && p[1] >= MIN(l.scoords[1], l.ecoords[1])) {
            return 1;
        }
    } else if (l.scoords[1] == l.ecoords[1] && l.scoords[1] == p[1]) {
        if (p[0] <= MAX(l.scoords[0], l.ecoords[0]) && p[0] >= MIN(l.scoords[0], l.ecoords[0])) {
            return 1;
        }
    }
    return 0;
}

char
blocked(int p[2], line* ls, size_t lc, sand* ss, size_t sc)
{
    for (int i = 0; i < lc; ++i) {
        if (in_line(p, ls[i])) {
            return 1;
        }
    }

    for (int i = 0; i < sc; ++i) {
        if (ss[i].p[0] == p[0] && ss[i].p[1] == p[1]) {
            return 1;
        }
    }

    return 0;
}

int
main()
{
    line lines[2000];
    static size_t linec;
    sand sands[10000];
    static size_t sandc;

    const char* in;
    char* line = NULL;
    size_t s = 0;

    int sx, sy, ex, ey;
    int lowesty = 0;

    while (getline(&line, &s, stdin) != EOF) {
        sx = -1;
        for (in = strtok(line, " -> "); in && *in;) {
            sscanf(in, "%d,%d", &ex, &ey);
            in = strtok(NULL, " -> ");

            if (ey > lowesty) {
                lowesty = ey;
            }

            if (sx != -1) {
                lines[linec].scoords[0] = sx;
                lines[linec].scoords[1] = sy;
                lines[linec].ecoords[0] = ex;
                lines[linec++].ecoords[1] = ey;
            }

            sx = ex;
            sy = ey;
        }
    }

    int x = 500, y = 0;

    while (y <= lowesty) {
        int p[] = {x, y+1};
        if (!blocked(p, lines, linec, sands, sandc)) {
            y += 1;
            continue;
        }

        p[0] = x-1;
        if (!blocked(p, lines, linec, sands, sandc)) {
            y += 1;
            x -= 1;
            continue;
        }

        p[0] = x+1;
        if (!blocked(p, lines, linec, sands, sandc)) {
            y += 1;
            x += 1;
            continue;
        }

        sands[sandc].p[0] = x;
        sands[sandc++].p[1] = y;
        x = 500;
        y = 0;
    }

    printf("%ld\n", sandc);

    return 0;
}
