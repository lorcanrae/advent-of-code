import time

from pprint import pprint
from copy import deepcopy


def parse_data(path: str) -> list:
    with open(path, "r") as f:
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


def has_horizontal(grid: list, old=0) -> int:
    for i in range(1, len(grid)):
        # select rows mirror is between
        row_up, row_down = i - 1, i
        # Assume that two rows match
        matched = True

        for j in range(len(grid)):
            # Check to see if we've reached the grid, if we have then break
            if row_up - j < 0 or row_down + j > len(grid) - 1:
                break

            # Comparison between rows - if not equal change flag to False
            if grid[row_up - j] != grid[row_down + j]:
                matched = False

        # If there is a valid reflection and it's not the same as a chosed row index input,
        # return row index. Other wise continue to look for a valid reflection
        if matched and i != old:
            return i

    # Return 0 if not valid reflection found
    return 0


def has_vertical(grid: list, old=0) -> int:
    # Transpose grid for ease of checking
    grid_T = list(zip(*grid))
    return has_horizontal(grid_T, old)


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


def c2(data: list) -> int:
    summary = 0

    # Iterate over all grids
    for k, grid in enumerate(data):

        # Set index's for original horizontal and vertical reflections
        old_h = has_horizontal(grid)
        old_v = has_vertical(grid)

        found_smudge = False

        # Iterate through each element in the grid and change to the opposite
        for i, row in enumerate(grid):
            for j, element in enumerate(row):
                new_grid = deepcopy(grid)
                new_grid[i][j] = "#" if element == "." else "."

                # Check new grid has valid horizontal reflection that != original
                hr = has_horizontal(new_grid, old_h)
                if hr:
                    # print(f"found smudge on grid {k} at location {i, j} that creates HORIZONTAL reflection on row {hr}")
                    # pprint(new_grid)
                    found_smudge = True
                    summary += hr * 100
                    break

                # Check new grid has valid vertical reflection that != original
                vr = has_vertical(new_grid, old_v)
                if vr:
                    # print(f"found smudge on grid {k} at location {i, j} that creates VERTICAL reflection on column {vr}")
                    found_smudge = True
                    summary += vr
                    break

            if found_smudge:
                break

    print(summary)
    return summary


if __name__ == "__main__":
    data = parse_data("input.txt")

    st = time.time()
    c1(data)
    print(f"Part 1 wall time: {time.time() - st: 0.5f} seconds\n")

    st = time.time()
    c2(data)
    print(f"Part 2 wall time: {time.time() - st: 0.5f} seconds\n")
