from datetime import datetime
from pprint import pprint


def read_data(path_: str) -> list:

    with open(path_, "r") as f:
        lines = f.read()

    lines = lines.split("\n\n")

    out_data = []
    for i, part in enumerate(lines):
        if i == 0:
            # out_data.append(part.split(":")[-1].strip().split(" "))
            partial_clean = part.split(":")[-1].strip().split(" ")
            out_data.append([int(num) for num in partial_clean])
        else:
            partial_clean = part.split(":")[-1].strip().split("\n")
            partial_clean = [
                [int(num) for num in seq.split(" ")] for seq in partial_clean
            ]
            out_data.append(partial_clean)

    return out_data


def c1(data: list) -> int:

    seed_out = []

    # Iterate through each seed
    for seed in data[0]:
        # iterate through each map
        for mapping in data[1:]:
            # iterate through each row in the map
            for row in mapping:
                # Check to see if seed is in a particular mapping row
                if seed >= row[1] and seed <= row[1] + row[2]:
                    # Calc offset
                    offset = row[0] - row[1]

                    # Mutate seed
                    seed += offset
                    # break out of the row iter to go to next mapping
                    break

        # Append seed value to output list
        seed_out.append(seed)

    closest = min(seed_out)
    print(closest)
    return closest


def c2():
    pass


if __name__ == "__main__":
    data = read_data("input.txt")
    # pprint(data)
    c1(data)
