import re
import math
from pprint import pprint
import time


def parse_data(path: str) -> list:
    with open(path, "r") as f:
        lines = [line.strip() for line in f.readlines() if line.strip() != ""]

    instructions = [0 if c == "L" else 1 for c in lines[0]]

    mapping = {}
    pattern = r"[A-Z0-9]{3}"

    for line in lines[1:]:
        regex_ = re.findall(pattern, line)
        mapping[regex_[0]] = regex_[1:]

    return instructions, mapping


def c1(data):
    instructions, mappings = data
    steps = 0
    location = "AAA"

    while location != "ZZZ":
        location = mappings[location][instructions[steps % len(instructions)]]
        steps += 1

    print(steps)
    return steps


def c2(data):
    instructions, mappings = data

    starts = [k for k in mappings.keys() if k[-1] == "A"]
    steps_per_start = []

    for start in starts:
        location = start
        steps = 0

        while location[-1] != "Z":
            location = mappings[location][instructions[steps % len(instructions)]]
            steps += 1

        steps_per_start.append(steps)

    min_cycles = math.lcm(*steps_per_start)

    print(min_cycles)
    return min_cycles


if __name__ == "__main__":
    data = parse_data("input.txt")

    st = time.time()
    c1(data)
    print(f"{time.time() - st: 0.5f}")

    st = time.time()
    c2(data)
    print(f"{time.time() - st: 0.5f}")
