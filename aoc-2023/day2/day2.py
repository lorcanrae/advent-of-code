def read_data(path_: str):

    with open(path_, "r") as f:
        lines = [line.split(":")[-1].strip() for line in f.readlines()]

    lines = [dice.split(";") for dice in lines]

    return lines


def c1(data: list) -> int:

    score = 0

    for i, game in enumerate(data, 1):
        r = 0
        g = 0
        b = 0
        valid = True

        for pull in game:
            for dice in pull.split(","):
                if "red" in dice:
                    r = max(int(dice.split()[0]), r)
                elif "green" in dice:
                    g = max(int(dice.split()[0]), g)
                elif "blue" in dice:
                    b = max(int(dice.split()[0]), b)

            if r > 12 or g > 13 or b > 14:
                valid = False

        if valid:
            score += i
    return score


def c2(data: list) -> int:

    power = 0

    for game in data:
        r = []
        g = []
        b = []

        for pull in game:
            for dice in pull.split(","):
                if "red" in dice:
                    r.append(int(dice.split()[0]))
                elif "green" in dice:
                    g.append(int(dice.split()[0]))
                elif "blue" in dice:
                    b.append(int(dice.split()[0]))

        power += max(r) * max(g) * max(b)

    return power


if __name__ == "__main__":

    data = read_data("day2_input.txt")
    print(c2(data))
