def get_data():
    with open('input.txt', 'r') as f:
        lines = [line.strip('\n').split(',') for line in f]
    return lines

def part1(data):
    # Checking each pair in input
    counter = 0
    for pair in data:
        # Convert n-m to sequential set for each elf, list of sets per pair
        sequential_zones = []
        for elf in pair:
            sec_start_stop = [int(zone) for zone in elf.split('-')]
            sequential_zones.append(set(range(sec_start_stop[0], sec_start_stop[1] + 1)))

        # Find overlapping elements and check if overlap == elf1 or elf2 set
        # Could also use .issubset() and .issuperset()
        overlap = sequential_zones[0].intersection(sequential_zones[1])
        if overlap == sequential_zones[0] or overlap == sequential_zones[1]:
            counter += 1

    print(counter)
    return counter

def part2(data):
    # Checking each pair in input
    counter = 0
    for pair in data:
        # Convert n-m to sequential set for each elf, list of sets per pair
        sequential_zones = []
        for elf in pair:
            sec_start_stop = [int(zone) for zone in elf.split('-')]
            sequential_zones.append(set(range(sec_start_stop[0], sec_start_stop[1] + 1)))

        # Find overlapping elements and check if there is any overlap
        overlap = sequential_zones[0].intersection(sequential_zones[1])
        if len(overlap) > 0:
            counter += 1

    print(counter)
    return counter

if __name__ == '__main__':
    part1(get_data())
    part2(get_data())
