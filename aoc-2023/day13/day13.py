from pprint import pprint
from copy import deepcopy

def parse_data(path: str) -> list:
    with open(path, "r") as f:
        # grids = [p.split("\n\n") for p in f.read()]
        grids = f.read().split("\n\n")

    grids = [p.split("\n") for p in grids]
    clean_grids = []
    for grid in grids:
        clean_grid = []
        for line in grid:
            if line:
                clean_grid.append([c for c in line])
        clean_grids.append(clean_grid)

    # pprint(clean_grids)

    return clean_grids


def has_horizontal(grid:list) -> int:

    for i in range(1, len(grid)):
        # select rows mirror is between
        row_up, row_down = i - 1, i

        for j in range(len(grid)):
            if row_up - j < 0 or row_down + j > len(grid) - 1:
                return i
            if grid[row_up - j] != grid[row_down + j]:
                break
    return 0


def has_vertical(grid: list) -> int:
    # Transpose grid for ease of checking
    grid_T = list(zip(*grid))
    for i in range(1, len(grid_T)):
        # select rows mirror is between
        row_up, row_down = i - 1, i

        for j in range(len(grid_T)):
            # print(j)
            if row_up - j < 0 or row_down + j > len(grid_T) - 1:
                return i
            if grid_T[row_up - j] != grid_T[row_down + j]:
                break
    return 0

def c1(data: list) -> int:

    summary = 0
    # Iterate over grids
    for grid in data:

        ### Horizontal Reflection
        h_factor = 100
        summary += has_horizontal(grid) * h_factor

        ### Vertical Reflection
        v_factor = 1
        summary += has_vertical(grid) * v_factor

    print(summary)
    return summary


def c2(data:list) -> int:
    summary = 0

    # Placeholder to iterate over multiple grids
    for k, grid in enumerate(data):
        print(f"looking at grid {k}")
        # pprint(grid)

        horizontal = has_horizontal(grid)
        vertical = has_vertical(grid)

        found_smudge = False
        for i, row in enumerate(grid):
            for j, element in enumerate(row):
                if found_smudge:
                    break
                new_grid = deepcopy(grid)
                new_grid[i][j] = "#" if element == "." else "."

                hr = has_horizontal(new_grid)
                if hr and hr != horizontal:
                    print(f"found smudge on grid {k} at location {i, j} that creates HORIZONTAL reflection on row {hr}")
                    # pprint(new_grid)
                    found_smudge = True
                    summary += hr * 100
                    break

                vr = has_vertical(new_grid)
                if vr and vr != vertical:
                    print(f"found smudge on grid {k} at location {i, j} that creates VERTICAL reflection on column {vr}")
                    found_smudge = True
                    summary += vr
                    break

            if found_smudge:
                break
        # if found_smudge:
        #     break

    print(summary)


if __name__ == "__main__":
    data = parse_data("input.txt")

    # c1(data)

    # print(has_vertical(data[0]), has_horizontal(data[0]))
    # print(has_vertical(data[1]), has_horizontal(data[1]))
    c2(data)
