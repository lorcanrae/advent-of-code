from pprint import pprint

def parse_data(path: str):
    with open(path, "r") as f:
        lines = [[c for c in l.strip()] for l in f.readlines()]

    return lines

def c1(data: list) -> int:

    galaxies = data.copy()

    # Add additional rows
    v_offset = 0
    for i in range(len(galaxies)):
        if "#" not in galaxies[i + v_offset]:
            # print(galaxies[i + v_offset])
            # print(i + v_offset)
            galaxies.insert(i + v_offset, ["."] * len(galaxies[0]))
            v_offset += 1
            # vert_increase.append(i)
        if i + v_offset == len(galaxies) - 1:
            break

    # Additional columns
    galaxies_T = list(zip(*galaxies))
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
            x1, y1 = galaxy_locs[i]
            x2, y2 = galaxy_locs[j]
            dist = abs(x1 - x2) + abs(y1 - y2)
            distances.append(dist)

    # pprint(distances)
    pprint(sum(distances))




if __name__ == "__main__":
    data = parse_data("input.txt")
    # pprint(data)

    c1(data)
