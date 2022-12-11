ins = [x.split() for x in open(0).read().split("\n") if x != '']
report = [20, 60, 100, 140, 180, 220]
s = 0
cs = 0
x = 1

screen = [['.'] * 40] *6
screen = []

for yc in range(6):
    screen.append([])
    for xc in range(40):
        screen[yc].append('.')

def draw():
    if ((cs-1) % 40)+1 in [x-1, x, x+1]:
        screen[(cs-1) // 40][((cs-1)%40)] = '#'


draw()
for i in ins:
    if i[0] == "noop":
        draw()
        cs += 1
    elif i[0] == "addx":
        draw()
        cs += 1

        draw()
        cs += 1

        x += int(i[1])

print("\n".join([ "".join(x) for x in screen]))
