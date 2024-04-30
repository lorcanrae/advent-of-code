def get_data():
    with open("input.txt", "r") as f:
        lines = [line.strip("\n") for line in f]
    return lines


def part1(lines):
    cycle = 0
    saved_signals = []
    X = 1

    for line in lines:
        if line == "noop":
            cycle += 1
            if (cycle - 20) % 40 == 0:
                saved_signals.append(X * cycle)
        else:
            for i in range(2):
                cycle += 1
                if (cycle - 20) % 40 == 0:
                    saved_signals.append(X * cycle)
            X += int(line.split()[1])

    print(sum(saved_signals))
    return sum(saved_signals)


def part2(lines):
    X = 1
    crt_pos = 0
    crt_out = ""

    for line in lines:
        if line == "noop":
            crt_out += "#" if crt_pos in [X - 1, X, X + 1] else "."

            if crt_pos == 39:
                print(crt_out)
                crt_out = ""
                crt_pos = 0
            else:
                crt_pos += 1

        else:
            for i in range(2):
                crt_out += "#" if crt_pos in [X - 1, X, X + 1] else "."

                if crt_pos == 39:
                    print(crt_out)
                    crt_out = ""
                    crt_pos = 0
                else:
                    crt_pos += 1
            X += int(line.split()[1])


if __name__ == "__main__":
    part1(get_data())
    part2(get_data())
