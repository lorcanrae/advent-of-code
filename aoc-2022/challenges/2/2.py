def part1(data):
    mapping_dict = {"A": "Y", "B": "Z", "C": "X"}
    mapping_dict_exact = {"A": "X", "B": "Y", "C": "Z"}
    score_dict = {"X": 1, "Y": 2, "Z": 3}
    score = 0
    for round in data:
        if round[-1] == mapping_dict[round[0]]:
            score += 6 + score_dict[round[-1]]
        elif round[-1] == mapping_dict_exact[round[0]]:
            score += 3 + score_dict[round[-1]]
        else:
            score += score_dict[round[-1]]
    return score


def part2(data):
    score_dict = {"A": 1, "B": 2, "C": 3}
    beat_by_dict = {"A": "B", "B": "C", "C": "A"}
    beats_dict = {"A": "C", "B": "A", "C": "B"}
    score = 0
    for round in data:
        if round[-1] == "X":
            score += score_dict[beats_dict[round[0]]]
        elif round[-1] == "Y":
            score += 3 + score_dict[round[0]]
        else:
            score += 6 + score_dict[beat_by_dict[round[0]]]
    return score


if __name__ == "__main__":
    with open("input.txt") as f:
        lines = [line.rstrip() for line in f]
    print(part1(lines))
    print(part2(lines))
