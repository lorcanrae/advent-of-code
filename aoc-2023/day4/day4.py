from datetime import datetime


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
        game_win_count = len(set(win_nums).intersection(set(game_nums)))
        match game_win_count:
            case 0:
                score += 0
            case 1:
                score += 1
            case _:
                score += 2 ** (game_win_count - 1)
    print(score)
    return score


def c2(data):
    """Really not a very efficient solution"""
    t1 = datetime.now()
    # Create list to store the number of runs per game, default 1 for every game
    runs_per_game = [1] * len(data)

    # Iterate over each game
    for game_number, game_runs in enumerate(runs_per_game):
        winning_numbers, game_numbers = data[game_number]

        # length of intersect to find number of wins - may be inefficient
        game_win_count = len(set(winning_numbers).intersection(set(game_numbers)))

        # if the game had winners, iterate over future to add wins
        if game_win_count > 0:
            for i in range(game_number + 1, game_number + game_win_count + 1):
                # Check to see if at the end of the list
                if i < len(data):
                    # Add number of runs to the runs_per_game - it's commutative
                    runs_per_game[i] += 1 * game_runs

    # Sum total number of runs/scratch cards
    total_runs = sum(runs_per_game)
    print(total_runs)
    print(datetime.now() - t1)
    return total_runs


if __name__ == "__main__":
    data = read_data("input.txt")
    c2(data)
