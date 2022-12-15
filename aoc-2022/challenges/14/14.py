import numpy as np

def move_sand(cave, pos):
    # directly below
    if cave[pos[0]+1, pos[1]] == '.':
        cave[pos[0], pos[1]] = '.'
        cave[pos[0]+1, pos[1]] = 'o'
        return cave, [pos[0]+1, pos[1]]
    # to the left
    elif cave[pos[0]+1, pos[1]-1] == '.':
        cave[pos[0], pos[1]] = '.'
        cave[pos[0]+1, pos[1]-1] = 'o'
        return cave, [pos[0]+1, pos[1]-1]
    # to the right
    elif cave[pos[0]+1, pos[1]+1] == '.':
        cave[pos[0], pos[1]] = '.'
        cave[pos[0]+1, pos[1]+1] = 'o'
        return cave, [pos[0]+1, pos[1]+1]
    # No where to go, start again
    cave[0, 500] = 'o' # need for p2
    return cave, [0, 500]

def get_cave():
    with open('input.txt', 'r') as f:
        data = [[coords.split(',') for coords in l.strip().split(' -> ')] for l in f.readlines()]

    cave = np.full((1000, 1000), '.')

    for row in data:
        for i in range(len(row) - 1):
            x1, x2 = int(row[i][1]), int(row[i+1][1])
            x1, x2 = (x2, x1) if x1 >= x2 else (x1, x2)
            y1, y2 = int(row[i][0]), int(row[i+1][0])
            y1, y2 = (y2, y1) if y1 >= y2 else (y1, y2)
            # print(x1, x2, y1, y2)
            cave[x1:x2+1, y1:y2+1] = '#'

    return cave

def part1(cave):
    lowest = np.argwhere(cave=='#')[:,0].max()
    position = (0, 500)

    while True:
        cave, position = move_sand(cave, position)
        if position[0] > lowest:
            break

    sand_count = np.count_nonzero(cave=='o') - 1
    print(sand_count)
    return sand_count

def part2(cave):
    # add floor to cave
    lowest = np.argwhere(cave=='#')[:,0].max()
    cave[lowest + 2, :] = '#'
    position = (0, 500)

    while True:
        cave, position = move_sand(cave, position)
        if cave[0, 500] == 'o' and cave[1,500] == 'o' and cave[1,499] == 'o' and cave[1,501] == 'o':
            break

    sand_count = np.count_nonzero(cave=='o')
    print(sand_count)
    return sand_count

if __name__ == '__main__':
    part1(get_cave())
    part2(get_cave())
