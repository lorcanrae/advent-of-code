import time
from pprint import pprint


def read_data(path_: str) -> list:

    with open(path_, "r") as f:
        lines = f.read().split("\n\n")

    out_data = []
    for i, part in enumerate(lines):
        if i == 0:
            partial_clean = part.split(":")[-1].strip().split(" ")
            out_data.append([int(num) for num in partial_clean])
        else:
            partial_clean = part.split(":")[-1].strip().split("\n")
            partial_clean = [
                [int(num) for num in seq.split(" ")] for seq in partial_clean
            ]
            out_data.append(partial_clean)

    return out_data

def seed_location(data, seed):
    # Just a helper function to give the location for a given input seed
    for mapping in data[1:]:
        for row in mapping:
            if row[1] <= seed <= (row[1] + row[2]):
                offset = row[0] - row[1]
                seed += offset
                break
    return seed

def c1(data:list) -> int:
    # Just a very large number
    closest_loc = 1e20

    # Iterate over seeds
    for seed in data[0]:
        seed_loc = seed_location(data, seed)

        # Update closest location if current seed is in fact closer
        closest_loc = seed_loc if seed_loc < closest_loc else closest_loc

    print(closest_loc)
    return closest_loc


def c2(data: list) -> int:

    ### First approximate answer
    approx_loc = [1e20, None, None]

    for i in range(0, len(data[0]), 2):
        start_range = data[0][i]
        end_range = data[0][i] + data[0][i + 1]

        step = int(data[0][i + 1] ** (1/2))

        for seed in range(start_range, end_range, step):
            seed_loc = seed_location(data, seed)
            if seed_loc < approx_loc[0]:
                approx_loc = (seed_loc, i, seed)

    ### Now look around approximate answer for exact answer
    closest_loc = 1e20
    closest_seed = None

    # Define the range to search in
    start_range = data[0][approx_loc[1]]
    end_range = data[0][approx_loc[1]] + data[0][approx_loc[1] + 1]
    step = int(data[0][approx_loc[1] + 1] ** (1/2))

    # Refine the search range even more
    start_range = max(start_range, approx_loc[2] - step - 1)
    end_range = min(end_range, approx_loc[2] + step + 1)

    # Search refined range
    for seed in range(start_range, end_range):
        seed_loc = seed_location(data, seed)

        if seed_loc < closest_loc:
            closest_loc = seed_loc
            closest_seed = seed

    # The solution is 1 too high?
    print(closest_loc, closest_seed)
    return closest_loc, closest_seed


def brute_force(data):
    closest_loc = [1e20, None, None]

    processed = 0

    num_seeds = sum(data[0])

    for i in range(0, len(data[0]), 2):
        start_range = data[0][i]
        end_range = data[0][i] + data[0][i + 1]

        for seed in range(start_range, end_range):
            seed_loc = seed_location(data, seed)
            if seed_loc < closest_loc[0]:
                closest_loc = (seed_loc, i, seed)

        processed += data[0][i]
        print(f"Processed {processed} out of {num_seeds} - {processed / num_seeds * 100:0.2f}%")
        print(f"Processed {i // 2 + 1} of {len(data[0]) // 2}.")


    print(closest_loc)
    return closest_loc


if __name__ == "__main__":
    data = read_data("input.txt")

    print("\nChallenge 1")
    st = time.time()
    c1(data)
    print(f"Wall time: {time.time() - st:0.3f} seconds")

    print("\nChallenge 2") # takes ~1 second
    st = time.time()
    c2(data)
    print(f"Wall time: {time.time() - st:0.3f} seconds")

    # print("\nChallenge 2 - Brute Force") # takes ~3,5 hours
    # st = time.time()
    # brute_force(data)
    # print(f"Wall time: {time.time() - st:0.3f} seconds")
