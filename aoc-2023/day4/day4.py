def read_data(path_: str) -> list:

    with open(path_, "r") as f:
        lines = [line.split(":")[1].strip() for line in f.readlines()]
    out_data = []
    for line in lines:
        game_nums = []
        for game in line.split("|"):
            game_nums.append(game.strip().replace("  ", " ").split(" "))
        out_data.append(game_nums)

    return out_data

def c1(data):
    score = 0
    for win_nums, game_nums in data:
        game_win_count = 0
        for num in win_nums:
            if num in game_nums:
                game_win_count += 1
        match game_win_count:
            case 0: score += 0
            case 1: score += 1
            case _: score += 2 ** (game_win_count - 1)
    print(score)
    return score

def c2(data):
    """Really not a very efficient solution"""
    # Create list to store the number of runs per game, defaul 1 for every game
    runs_per_game = [1] * len(data)

    #
    for game_number, game_runs in enumerate(runs_per_game):
        winning_numbers, game_numbers = data[game_number]
        # Repeat experiment for the number of runs
        for _ in range(game_runs):
            game_win_count = len(set(winning_numbers).intersection(set(game_numbers)))
            if game_win_count > 0:
                for i in range(game_number+1, game_number + game_win_count+1):
                    if i < len(data):
                        runs_per_game[i] += 1


    total_runs = sum(runs_per_game)
    print(runs_per_game)
    print(total_runs)
    return total_runs


if __name__ == "__main__":
    data = read_data("input.txt")
    c2(data)
