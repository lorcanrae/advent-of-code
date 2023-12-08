import re
from pprint import pprint
import time

def parse_data(path: str) -> list:
    with open(path, "r") as f:
        lines = [line.strip() for line in f.readlines() if line.strip() != ""]

    instructions = lines[0]

    mapping = {}
    pattern = r"[A-Z]{3}"

    for line in lines[1:]:
        regex_ = re.findall(pattern, line)
        mapping[regex_[0]] = regex_[1:]

    return instructions, mapping

def c1(data):
    instructions, mapping = data

    counter = 0

    location = "AAA"

    while location != "ZZZ":
        for instruction in instructions:
            location = mapping[location][0] if instruction == "L" else mapping[location][1]
            counter += 1
            if location == "ZZZ":
                break

    print(counter)
    return counter


def c2(data):
    instructions, mappings = data

    starts = [k for k in mappings.keys() if k[-1] == "A"]

    print(starts)

    # for k in mappings.keys():
    #     start = k if k[-1] == "A" else 0





if __name__ == "__main__":
    data = parse_data("test2.txt")

    st = time.time()
    c2(data)
    print(time.time() - st)
