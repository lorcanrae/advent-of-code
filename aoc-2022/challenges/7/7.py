def get_data():
    with open("input.txt", "r") as f:
        data = [line.strip("\n") for line in f.readlines()]
    return data


def create_dirs(data):
    dirs = {}

    for line in data:
        match line.split():
            case "$", "cd", "/":
                cur_path = ["/"]
            case "$", "cd", "..":
                cur_path.pop()
            case "$", "cd", x:
                cur_path.append(x + "/")
            case "dir", _:
                pass
            case "$", "ls":
                pass
            case size, _:
                for i, _ in enumerate(cur_path, 1):
                    try:
                        dirs["".join(cur_path[:i])] += int(size)
                    except:
                        dirs["".join(cur_path[:i])] = int(size)
    return dirs


def part1(dirs):
    sum_folders = sum(value for value in dirs.values() if value <= 100_000)
    print(sum_folders)
    return sum_folders


def part2(dirs, total_space, req_space):
    max_other = total_space - req_space
    min_del = min(
        dir_size for dir_size in dirs.values() if dir_size >= dirs["/"] - max_other
    )
    print(min_del)
    return min_del


if __name__ == "__main__":
    dirs = create_dirs(get_data())

    part1(dirs)
    part2(dirs, 70_000_000, 30_000_000)
