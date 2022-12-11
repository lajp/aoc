ip = [x.split() for x in open(0).read().split("\n") if x != '']

knots = [(0, 0)] * 10
tcoords = {}

for line in ip:
    for i in range(0, int(line[1])):
        (hx, hy) = knots[0]
        if line[0] == 'L':
            hx -= 1
        elif line[0] == 'R':
            hx += 1
        elif line[0] == 'U':
            hy -= 1
        elif line[0] == 'D':
            hy += 1

        knots[0] = (hx, hy)

        for i in range(1, len(knots)):
            (hx, hy) = knots[i-1]
            (tx, ty) = knots[i]

            if hx-tx > 1:
                if hy > ty:
                    ty += 1
                elif hy < ty:
                    ty -= 1
                tx += 1

            if tx-hx > 1:
                if hy > ty:
                    ty += 1
                elif hy < ty:
                    ty -= 1
                tx -= 1

            if hy-ty > 1:
                if hx > tx:
                    tx += 1
                elif hx < tx:
                    tx -= 1
                ty += 1

            if ty-hy > 1:
                if hx > tx:
                    tx += 1
                elif hx < tx:
                    tx -= 1
                ty -= 1

            if i == 9:
                tcoords[(tx, ty)] = True

            knots[i] = (tx, ty)

print(len(tcoords))
