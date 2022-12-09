import numpy as np

def get_data():
    with open('input.txt', 'r') as f:
        tree_grid = [[int(n) for n in line.strip('\n')] for line in f.readlines()]
    return tree_grid

def part1(data):
    tree_grid = np.array(data)
    visible = 0
    for i in range(1, tree_grid.shape[0] - 1):
        for j in range(1, tree_grid.shape[1] - 1):

            right = tree_grid[i, j] > np.max(tree_grid[i, :j])
            left = tree_grid[i, j] > np.max(tree_grid[i, j + 1:])
            top = tree_grid[i, j] > np.max(tree_grid[:i, j])
            bottom = tree_grid[i, j] > np.max(tree_grid[i + 1:, j])
            visible += right or left or top or bottom

    visible += 2 * sum(tree_grid.shape) - 4
    print(visible)
    return visible

def part2(data):
    tree_grid = np.array(data)
    # Helper
    def calc_score(cur_tree, line_of_trees):
        # I am unhappy this function looks this way
        score = 0
        for tree_height in line_of_trees:
            if cur_tree > tree_height:
                score += 1
            else:
                score += 1
                return score
        return score
    scores = np.zeros(tree_grid.shape)

    for i in range(tree_grid.shape[0]):
        for j in range(tree_grid.shape[1]):
            cur_tree_height = tree_grid[i, j]
            score = 1
            # from current tree
            # looking right
            score *= calc_score(cur_tree_height, tree_grid[i, j+1:])
            # looking left
            score *= calc_score(cur_tree_height, tree_grid[i, :j][::-1])
            # looking up
            score *= calc_score(cur_tree_height, tree_grid[:i, j][::-1])
            # looking down
            score *= calc_score(cur_tree_height, tree_grid[i+1:, j])

            scores[i, j] = score

    print(int(np.max(scores)))
    return int(np.max(scores))

if __name__ == '__main__':
    part1(get_data())
    part2(get_data())
