def get_data():
    with open('input.txt', 'r') as f:
        data = f.readlines()[0]
    return data

def part1(data):
    for i, _ in enumerate(data, 1):
        if len(set(data[i - 4: i])) == 4:
            print(i)
            return i


def part2(data):
    for i, _ in enumerate(data, 1):
        if len(set(data[i - 14: i])) == 14:
            print(i)
            return i


if __name__ == '__main__':
    part1(get_data())
    part2(get_data())
