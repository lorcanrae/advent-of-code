import re
from functools import reduce
import time


def read_data(path_: str) -> list:

    with open(path_, "r") as f:
        lines = [line.strip() for line in f.readlines()]

    pattern = r"\d{1,4}"

    times = [int(num) for num in re.findall(pattern, lines[0])]
    distances = [int(num) for num in re.findall(pattern, lines[1])]

    return times, distances


def c1(data: list) -> int:
    times, distances = data
    number_of_ways = []

    for i, ms in enumerate(times):
        counter = 0
        for j in range(0, ms + 1):
            time = ms - j
            distance = j * time
            counter += 1 if distance > distances[i] else 0

        number_of_ways.append(counter)

    sum_ways = reduce((lambda x, y: x * y), number_of_ways)
    print(sum_ways)
    return sum_ways


def c2(data: list) -> int:

    number_of_ways = 0
    total_time = int("".join([str(num) for num in data[0]]))
    total_distance = int("".join([str(num) for num in data[1]]))

    for i in range(0, total_time + 1):
        time = total_time - i
        distance = i * time
        number_of_ways += 1 if distance > total_distance else 0

    print(number_of_ways)
    return number_of_ways


if __name__ == "__main__":
    data = read_data("input.txt")
    print(data)

    st = time.time()
    c1(data)
    print(f"Wall time: {time.time() - st: 0.5f} seconds")

    st = time.time()
    c2(data)
    print(f"Wall time: {time.time() - st: 0.5f} seconds")
