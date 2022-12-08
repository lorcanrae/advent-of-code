import string

def get_data():
    with open("input.txt") as f:
        rucksacks = [rucksack[:-1] for rucksack in f.readlines()]

    lowercase_priority = {letter: i + 1 for i, letter in enumerate(string.ascii_lowercase)}
    upercase_priority = {letter: i + 27 for i, letter in enumerate(string.ascii_uppercase)}
    priority = {**lowercase_priority, **upercase_priority}
    return rucksacks, priority

def part1(rucksacks, priority):
    cumsum = 0
    for rucksack in rucksacks:
        first_compartment =  set(rucksack[:len(rucksack)//2])
        second_compartment = set(rucksack[len(rucksack)//2:])

        common_item = first_compartment.intersection(second_compartment).pop()
        cumsum += priority[common_item]

    return cumsum

def part2(rucksacks, priority):
    cumsum = 0
    for i in range(0, len(rucksacks), 3):
        rucksack_sets = []
        for j in range(i+1, i+3):
            rucksack = set(rucksacks[j])
            rucksack_sets.append(rucksack)

        common_item = set(rucksacks[i]).intersection(rucksack_sets[0], rucksack_sets[1]).pop()
        cumsum += priority[common_item]

    return cumsum

if __name__ == '__main__':
    data, prior = get_data()

    print(part1(data, prior))

    print(part2(data, prior))
