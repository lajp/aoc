packets = [map(eval, x.split("\n")) for x in (open(0).read() + "\n").split("\n\n") if x != '']
ans = 0

def check(a, b):
    # returns 0 if cannot tell
    # returns 1 if valid
    # returns -1 if invalid

    if isinstance(a, list):
        if isinstance(b, list):
            for i in range(max(len(a), len(b))):
                if i == len(a) and i < len(b):
                    return 1
                elif i == len(b) and i < len(a):
                    return -1

                r = check(a[i], b[i])
                if r != 0:
                    return r

            return 0
        else:
            return check(a, [b])

    elif isinstance(b, list):
        return check([a], b)

    else:
        if a < b:
            return 1
        elif a > b:
            return -1
        else:
            return 0

for (ind, pair) in enumerate(packets):
    (p1, p2) = tuple(pair)
    if check(p1, p2) == 1:
        ans += ind+1

print(ans)
