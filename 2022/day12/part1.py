import sys

sys.setrecursionlimit(10**6)

mp = [list(map(ord, x)) for x in open(0).read().split("\n") if x != '']

tx = 0
ty = 0
sx = 0
sy = 0

for j in range(len(mp)):
    for k in range(len(mp[0])):
        if mp[j][k] == ord('S'):
            tx = k
            ty = j
            mp[j][k] = ord('a')
        elif mp[j][k] == ord('E'):
            sx = k
            sy = j
            mp[j][k] = ord('z')

mp_vals = {(sx, sy): 0}


def recurse(x, y):
    hgt = mp[y][x]
    val = mp_vals[(x, y)]
    if x == tx and y == ty:
        return

    if x > 0:
        if mp[y][x-1] >= hgt-1:
            if (x-1, y) not in mp_vals or mp_vals[(x-1, y)] > val+1:
                mp_vals[(x-1, y)] = val+1
                recurse(x-1, y)

    if x < len(mp[0])-1:
        if mp[y][x+1] >= hgt-1:
            if (x+1, y) not in mp_vals or mp_vals[(x+1, y)] > val+1:
                mp_vals[(x+1, y)] = val+1
                recurse(x+1, y)

    if y > 0:
        if mp[y-1][x] >= hgt-1:
            if (x, y-1) not in mp_vals or mp_vals[(x, y-1)] > val+1:
                mp_vals[(x, y-1)] = val+1
                recurse(x, y-1)

    if y < len(mp)-1:
        if mp[y+1][x] >= hgt-1:
            if (x, y+1) not in mp_vals or mp_vals[(x, y+1)] > val+1:
                mp_vals[(x, y+1)] = val+1
                recurse(x, y+1)


recurse(sx, sy)
print(mp_vals[(tx, ty)])
