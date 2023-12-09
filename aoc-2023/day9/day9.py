from pprint import pprint
import time

def parse_data(path: str):
    with open(path, 'r') as f:
        lines = [[int(c) for c in l.strip().split()] for l in f.readlines()]

    return lines


def c1(data: list) -> int:

    next_vals = []

    for line in data:
        history = []
        history.append(line)

        ## Reduce
        while len(set(history[-1])) != 1 or list(set(history[-1]))[0] != 0:
            next_row = [j - i for i, j in zip(history[-1], history[-1][1:])]
            history.append(next_row)
            # print(next_row)

        ## Back up
        end_val = 0

        for step in history[::-1]:
            end_val = step[-1] + end_val
        next_vals.append(end_val)

    print(sum(next_vals))
    return sum(next_vals)

if __name__ == "__main__":
    data = parse_data("input.txt")

    st = time.time()
    c1(data)
    print(f"Wall time: {time.time() - st: 0.5f} seconds.")
