def get_data():
    with open('input.txt', 'r') as f:
        raw_directions = [line.strip('\n') for line in f]

    directions = []
    for direction in raw_directions:
        directions.append([int(element) for element in direction.split() if element.isnumeric()])

    crate_stacks = [
        ['H', 'T', 'Z', 'D'],
        ['Q', 'R', 'W', 'T', 'G', 'C', 'S'],
        ['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H'],
        ['L', 'C', 'N', 'F', 'H', 'Z'],
        ['G', 'L', 'F', 'Q', 'S'],
        ['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S'],
        ['Z', 'F', 'J'],
        ['D', 'L', 'V', 'Z', 'R', 'H', 'Q'],
        ['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D']
    ]

    return directions, crate_stacks

def part1(directions, crate_stacks):
    for direction in directions:
        for i in range(direction[0]):
            source = crate_stacks[direction[1] - 1]
            destination = crate_stacks[direction[2] - 1]

            destination.append(source.pop())
    top_crates = ''.join([crate_stack[-1] for crate_stack in crate_stacks])
    print(top_crates)
    return top_crates


def part2(directions, crate_stacks):
    for direction in directions:
        source = crate_stacks[direction[1] - 1]
        destination = crate_stacks[direction[2] - 1]
        num_crates = direction[0]
        for i in range(-num_crates, 0):
            destination.append(source.pop(i))
    top_crates = ''.join([crate_stack[-1] for crate_stack in crate_stacks])
    print(top_crates)
    return top_crates

if __name__ == '__main__':
    part1(*get_data())
    part2(*get_data())
