from pprint import pprint

def parse_data(path: str):
    with open(path, "r") as f:
        lines = [[c for c in l.strip()] for l in f.readlines()]

    return lines

def c1(data:list) -> int:

    # start with finding "S"
    for i in range(len(data)):
        for j in range(len(data[0])):
            if data[i][j] == "S":
                s = (i, j)
                # print(i, j)

    e = s
    steps = 0
    d = "s" # Should be the direction that you would enter the next tile from

    while True:
        if steps > 0 and s == e:
            break
        t = data[s[0]][s[1]]

        if t == "-":
            if d == "w":
                s = (s[0], s[1] + 1)
            else:
                s = (s[0], s[1] - 1)

        if t == "|":
            if d == "s":
                s = (s[0] - 1, s[1])
            else:
                s = (s[0] + 1, s[1])

        if t == "F" or t == "S": # Hardcoded for my input
            if d == "s":
                d = "w"
                s = (s[0] , s[1] + 1)
            else:
                d = "n"
                s = (s[0] + 1, s[1])

        if t == "7":
            if d == "s":
                d = "e"
                s = (s[0], s[1] - 1)
            else:
                d = "n"
                s = (s[0] + 1, s[1])

        if t == "J":
            if d == "n":
                d = "e"
                s = (s[0], s[1] - 1)
            else:
                d = "s"
                s = (s[0] - 1, s[1])

        if t == "L":
            if d == "n":
                d = "w"
                s = (s[0], s[1] + 1)
            else:
                d = "s"
                s = (s[0] - 1, s[1])

        steps += 1

    print(steps // 2)
    return steps // 2

def c2(data:list) -> int:

    # start with finding "S"
    for i in range(len(data)):
        for j in range(len(data[0])):
            if data[i][j] == "S":
                s = (i, j)
                # print(i, j)

    e = s
    steps = 0
    d = "s" # Should be the direction that you would enter the next tile from
    pipe = set()

    while True:
        if steps > 0 and s == e:
            break
        t = data[s[0]][s[1]]
        pipe.add(s)

        if t == "-":
            if d == "w":
                s = (s[0], s[1] + 1)
            else:
                s = (s[0], s[1] - 1)

        if t == "|":
            if d == "s":
                s = (s[0] - 1, s[1])
            else:
                s = (s[0] + 1, s[1])

        if t == "F" or t == "S": # Hardcoded for my input
            if d == "s":
                d = "w"
                s = (s[0] , s[1] + 1)
            else:
                d = "n"
                s = (s[0] + 1, s[1])

        if t == "7":
            if d == "s":
                d = "e"
                s = (s[0], s[1] - 1)
            else:
                d = "n"
                s = (s[0] + 1, s[1])

        if t == "J":
            if d == "n":
                d = "e"
                s = (s[0], s[1] - 1)
            else:
                d = "s"
                s = (s[0] - 1, s[1])

        if t == "L":
            if d == "n":
                d = "w"
                s = (s[0], s[1] + 1)
            else:
                d = "s"
                s = (s[0] - 1, s[1])

        steps += 1

    t = 0

    for i in range(len(data)):
        for j in range(len(data[0])):
            h, hh = 0, 0

            # If point is part of the pipe, we don't need to check it
            if (i, j) in pipe:
                continue

            # Check horizontally from point
            for jj in range(len(data[0])):

                # Checking to see if vertical points in path
                if (i, jj) in pipe and data[i][jj] in "|JL":
                    if jj > j:
                        h += 1
                    else:
                        hh += 1

            # Check to see if point is inside or outside - to the left is what we care about
            if hh % 2 == 1:
                t += 1
    print(t)





if __name__ == "__main__":

    data = parse_data("input.txt")

    # pprint(data)
    # print(data)

    c1(data)


    c2(data)
