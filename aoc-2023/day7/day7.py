from collections import Counter
from pprint import pprint


def parse_data(path: str):

    with open(path, "r") as f:
        lines = [line.strip().split() for line in f.readlines()]

    return lines


def c1(data: list) -> int:
    out = []

    for hand in data:
        count = sorted(Counter(hand[0]).values(), reverse=True)

        ind_cards = ["23456789TJQKA".index(card) for card in hand[0]]

        ind_hand_power = [
            [1, 1, 1, 1, 1],
            [2, 1, 1, 1],
            [2, 2, 1],
            [3, 1, 1],
            [3, 2],
            [4, 1],
            [5],
        ].index(count)

        out.append(tuple([ind_hand_power + 1] + ind_cards + [int(hand[1])]))

    rank = 0

    pprint(sorted(out))

    for i, hand in enumerate(sorted(out), 1):
        rank += i * hand[-1]

    print(rank)
    return rank


def c2(data):
    out = []

    for hand in data:
        count = Counter(hand[0])

        if "J" in count and count.get("J") != 5:
            if count.most_common()[0][0] != "J":
                count[count.most_common()[0][0]] += count.pop("J")
            else:
                count[count.most_common()[1][0]] += count.pop("J")

        count = sorted(count.values(), reverse=True)

        ind_cards = ["J23456789TQKA".index(card) for card in hand[0]]

        ind_hand_power = [
            [1, 1, 1, 1, 1],
            [2, 1, 1, 1],
            [2, 2, 1],
            [3, 1, 1],
            [3, 2],
            [4, 1],
            [5],
        ].index(count)

        out.append(tuple([ind_hand_power + 1] + ind_cards + [int(hand[1])]))

    rank = 0

    for i, hand in enumerate(sorted(out), 1):
        rank += i * hand[-1]

    print(rank)
    return rank


if __name__ == "__main__":
    data = parse_data("input.txt")
    # c1(data)
    c2(data)
