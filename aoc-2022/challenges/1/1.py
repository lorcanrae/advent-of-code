def load_data():
    # load data
    with open("input.txt", "r") as f:
        lines = [
            "break" if line == "\n" else line.strip("\n") for line in f.readlines()
        ]
        f.close()

    # clean - list of lists with calories for each item as strings
    each_elf = " ".join(lines).split(" break ")
    each_elf = [elf.split(" ") for elf in each_elf]

    return each_elf


def part1(data):
    data = [sum([int(cals) for cals in elf]) for elf in data]
    max_cals = sorted(data, reverse=True)[0]
    print(f"\nMax calories carried by a single elf is {max_cals}")
    return max_cals


def part2(data):
    data = [sum([int(cals) for cals in elf]) for elf in data]
    top_three_max_cals = sum(sorted(data, reverse=True)[:3])
    print(f"\Sum of calories carried by top three elves is {top_three_max_cals}")
    return top_three_max_cals


if __name__ == "__main__":
    data = load_data()
    p1 = part1(data)
    p2 = part2(data)
