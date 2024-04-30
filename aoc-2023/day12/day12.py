from pprint import pprint


def parse_data(path: str):
    with open(path, "r") as f:
        lines = [l.strip() for l in f.readlines()]

    pattern = [line.split(" ")[0] for line in lines]

    return lines


if __name__ == "__main__":
    data = parse_data("test.txt")
