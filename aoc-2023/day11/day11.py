from pprint import pprint
from copy import deepcopy


def parse_data(path: str):
    with open(path, "r") as f:
        lines = [[c for c in l.strip()] for l in f.readlines()]

    return lines


def c1(data: list) -> int:

    galaxies = deepcopy(data)

    # Add additional rows
    v_offset = 0
    for i, row in enumerate(data):
        if "#" not in row:
            galaxies.insert(i + v_offset, ["."] * len(galaxies[0]))
            v_offset += 1

    # Additional columns
    galaxies_T = list(zip(*data))
    h_offset = 0
    for i, column in enumerate(galaxies_T):
        if "#" not in column:
            for row_i in range(len(galaxies)):
                galaxies[row_i].insert(i + h_offset, ".")
            h_offset += 1

    # Find position of galaxies for comparison
    galaxy_locs = []
    for i, row in enumerate(galaxies):
        for j, val in enumerate(row):
            if val == "#":
                galaxy_locs.append((i, j))

    # Find manhattan distance between two galaxies
    distances = []
    for i in range(len(galaxy_locs)):
        for j in range(i + 1, len(galaxy_locs)):
            y1, x1 = galaxy_locs[i]
            y2, x2 = galaxy_locs[j]
            dist = abs(x1 - x2) + abs(y1 - y2)
            distances.append(dist)

    pprint(sum(distances))


def c2(data: list) -> int:
    pass

    data_copy = data.copy()

    v_rows = []
    for i, row in enumerate(data_copy):
        if "#" not in row:
            v_rows.append(i)

    h_rows = []
    for i, column in enumerate(list(zip(*data_copy))):
        if "#" not in column:
            h_rows.append(i)

    galaxy_locs = []
    for i, row in enumerate(data):
        for j, val in enumerate(row):
            if val == "#":
                galaxy_locs.append((i, j))

    distances = []
    for i in range(len(galaxy_locs)):
        for j in range(i + 1, len(galaxy_locs)):
            y1, x1 = galaxy_locs[i]
            y2, x2 = galaxy_locs[j]
            h_f = sum(
                [1 if x in range(min(x1, x2), max(x1, x2)) else 0 for x in h_rows]
            )
            v_f = sum(
                [1 if y in range(min(y1, y2), max(y1, y2)) else 0 for y in v_rows]
            )
            f = (
                1000000 - 1
            )  # Minus 1 to account for for the row that would already be there

            dist = abs(x1 - x2) + h_f * f + abs(y1 - y2) + v_f * f
            distances.append(dist)

    print(sum(distances))


if __name__ == "__main__":
    data = parse_data("input.txt")
    # pprint(data)

    # c1(data)

    c2(data)
