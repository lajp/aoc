ins = [x.split() for x in open(0).read().split("\n") if x != '']
report = [20, 60, 100, 140, 180, 220]
s = 0
cs = 0
x = 1

for i in ins:
    if i[0] == "noop":
        cs += 1
        if cs in report:
            s += cs*x
    elif i[0] == "addx":
        cs += 1
        if cs in report:
            s += cs*x

        cs += 1
        if cs in report:
            s += cs*x

        x += int(i[1])

print(s)
