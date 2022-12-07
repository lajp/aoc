lines = [x for x in open(0).read().split("\n") if x != ''][1:]

dirs = {}
cursize = 0
tree = ["/"]

for line in lines:
    if line[0] == "$":
        if line[2:4] == "cd":
            for i in range(1, len(tree[:-1])):
                d = "/" + "/".join(tree[1:-i])
                if d in dirs:
                    dirs[d] = dirs[d]+cursize
                else:
                    dirs[d] = cursize

            if "/" in dirs:
                dirs["/"] = dirs["/"]+cursize
            else:
                dirs["/"] = cursize

            if "/" + "/".join(tree[1:]) not in dirs.keys():
                dirs["/" + "/".join(tree[1:])] = cursize

            cursize = 0

            if line[5:] == "..":
                tree.pop()
            elif line[5:] == "/":
                tree.clear()
                tree.append("/")
            else:
                tree.append(line[5:])

    else:
        sp = line.split()
        if sp[0] != "dir":
            cursize += int(sp[0])

dirs["/" + "/".join(tree[1:])] = cursize
for i in range(len(tree[:-1])):
    d = "/" + "/".join(tree[1:-i])
    if d in dirs:
        dirs[d] = dirs[d]+cursize
    else:
        dirs[d] = cursize

dirs["/"] += cursize

sizes = []
for d in dirs:
    sizes.append(dirs[d])

sizes = sorted(sizes)
needed = 30000000-(70000000-dirs["/"])

for s in sizes:
    if s >= needed:
        print(s)
        break

