from pprint import pprint
import re






def parse_data(path: str) -> list:
    with open(path, "r") as f:
        # lines = [[e for e in l.strip()] for l in f.readlines()]
        lines = [l.strip() for l in f.readlines()]




    return lines






def c1(data: list) -> int:
    # Transpose

    data_T = list(map(list, zip(*data)))
    data_T = ["".join(x) for x in data_T]
    len_ = len(data_T[0])

    pprint(data_T)

    cube_pattern = r"#"
    round_pattern = r"O"

    cubes = [0] + [match.start() for match in re.finditer(cube_pattern, data_T[0])]
    rounds = [match.start() for match in re.finditer(round_pattern, data_T[0])]
    new_row = ["."] * len_

    for i, cube in enumerate(cubes):
        print(cube)
        # first cube
        if i == 0:
            section = new_row[:cube]

        # intermediate cube

        # last cube

        for round in rounds:
            if round < cube:
                pass

    print(cubes)
    print(rounds)


if __name__ == "__main__":
    data = parse_data("test.txt")

    pprint(data)

    c1(data)
