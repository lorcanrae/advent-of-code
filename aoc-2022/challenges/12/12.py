import numpy as np
import networkx as nx

# my graph theory game needs work

def get_data():
    with open('input.txt', 'r') as f:
        matrix = np.array([[ord(c) for c in l.strip()] for l in f.readlines()])
    return matrix

def part1_part2(matrix):

    # Start and end coordinates
    start = tuple(*np.argwhere(matrix==ord('S')))
    end = tuple(*np.argwhere(matrix==ord('E')))

    # Replace with value
    matrix[start] = ord('a')
    matrix[end] = ord('z')

    G = nx.grid_2d_graph(*np.array(matrix).shape, create_using=nx.DiGraph)
    G.remove_edges_from([(a, b) for a, b in G.edges if matrix[b] > matrix[a] + 1])
    p = nx.shortest_path_length(G, target=end)

    # Part 1
    from_start = p[start]

    # Part 2
    from_any_a = min(p[a] for a in p if matrix[a] == ord('a'))

    print(from_start, from_any_a)
    return from_start, from_any_a

if __name__ == '__main__':
    part1_part2(get_data())
