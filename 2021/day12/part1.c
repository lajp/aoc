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

int find_paths(cave* start, cave* end, cave** visited, int vc) {
    if(start == end) {
        return 1;
    }
    int ans = 0;
    cave** new_visited = malloc((vc+1)*sizeof(cave*));
    int nvc = 0;
    if(visited && vc > 0) {
        for(int i = 0; i < vc; ++i) {
            new_visited[nvc++] = visited[i];
        }
    }
    if(start->small) {
        new_visited[nvc] = start;
        nvc++;
    }
    for(int i = 0; i < start->connc; ++i) {
        if(!cave_visited(start->conn[i], new_visited, nvc)) {
            ans += find_paths(start->conn[i], end, new_visited, nvc);
        }
    }
    return ans;
}

int main() {
    int ans = 0;
    char* line = NULL;
    size_t size = 128;
    cave* start = NULL;
    cave* end = NULL;
    cave caves[100];
    int cavec = 0;
    const char* in;
    while(getline(&line, &size, stdin) != EOF) {
        in = strtok(line, "-");
        cave* tmp = cave_with_name(caves, cavec, (char*)in);
        if(!tmp) {
            tmp = &caves[cavec++];
            tmp->name = malloc((strlen(in)+1)*sizeof(char));
            strcpy(tmp->name, in);
            tmp->small = islower(in[0]);
            tmp->connc = 0;
            tmp->conn = malloc(100*sizeof(struct cave*));
        }
        if(strcmp(in, "start") == 0) {
            start = tmp;
        }
        in = strtok(NULL, "\n");
        cave* con = cave_with_name(caves, cavec, (char*)in);
        if(!con) {
            con = &caves[cavec++];
            con->name = malloc((strlen(in)+1)*sizeof(char));
            strcpy(con->name, in);
            con->small = islower(in[0]);
            con->connc= 0;
            con->conn = malloc(100*sizeof(struct cave*));
            if(strcmp(in, "end") == 0) {
                end = con;
            }
        }
        tmp->conn[tmp->connc++] = con;
        con->conn[con->connc++] = tmp;
        free(line);
        line = NULL;
    }
    ans = find_paths(start, end, NULL, 0);
    printf("Answer: %d\n", ans);
    return 0;
}
