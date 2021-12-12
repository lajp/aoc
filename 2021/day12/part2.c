#include <stddef.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>

typedef struct cave {
    char* name;
    bool small;
    int connc;
    struct cave** conn;
} cave;

cave* cave_with_name(cave* c, int cc, char* n) {
    for(int i = 0; i < cc; ++i) {
        if(strcmp(c[i].name, n) == 0) {
            return &c[i];
        }
    }
    return NULL;
}

bool cave_visited(cave* c, cave** v, int vc) {
    for(int i = 0; i < vc; ++i) {
        if(v[i] == c) {
            return true;
        }
    }
    return false;
}

int find_paths(cave* start, cave* end, cave** visited, int vc, bool once) {
    if(start == end) {
        return 1;
    }
    int ans = 0;
    cave** new_visited = malloc((vc+(start->small))*sizeof(cave*));
    int nvc = 0;
    if(visited && vc > 0) {
        for(int i = 0; i < vc; ++i) {
            new_visited[nvc++] = visited[i];
        }
    }
    if(start->small) {
        new_visited[nvc++] = start;
    }
    for(int i = 0; i < start->connc; ++i) {
        if(strcmp(start->conn[i]->name, "start") == 0) {
            continue;
        }
        if(!cave_visited(start->conn[i], new_visited, nvc)) {
            ans += find_paths(start->conn[i], end, new_visited, nvc, once);
        } else if(!once) {
            ans += find_paths(start->conn[i], end, new_visited, nvc, 1);
        }
    }
    free(new_visited);
    return ans;
}

int main() {
    int ans = 0;
    cave* start = NULL;
    cave* end = NULL;
    cave caves[100];
    int cavec = 0;
    char in1[10], in2[10];
    while(scanf("%[^-]-%s\n", in1, in2) != EOF) {
        cave* tmp = cave_with_name(caves, cavec, (char*)in1);
        if(!tmp) {
            tmp = &caves[cavec++];
            tmp->name = malloc((strlen(in1)+1)*sizeof(char));
            strcpy(tmp->name, in1);
            tmp->small = islower(in1[0]);
            tmp->connc = 0;
            tmp->conn = malloc(100*sizeof(struct cave*));
        }
        if(strcmp(in1, "start") == 0) {
            start = tmp;
        }
        cave* con = cave_with_name(caves, cavec, (char*)in2);
        if(!con) {
            con = &caves[cavec++];
            con->name = malloc((strlen(in2)+1)*sizeof(char));
            strcpy(con->name, in2);
            con->small = islower(in2[0]);
            con->connc= 0;
            con->conn = malloc(100*sizeof(struct cave*));
            if(strcmp(in2, "end") == 0) {
                end = con;
            }
        }
        tmp->conn[tmp->connc++] = con;
        con->conn[con->connc++] = tmp;
    }
    ans = find_paths(start, end, NULL, 0, false);
    printf("Answer: %d\n", ans);
    return 0;
}
