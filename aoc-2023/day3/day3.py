import regex as re
import string


def read_data(path_: str):

    with open(path_, "r") as f:
        lines = [line.strip() for line in f.readlines()]

    return lines


def c1(data: list) -> int:

    total_sum = 0
    x_max = len(data[0])
    y_max = len(data)

    pattern = r"\d{1,3}"

    for i, row in enumerate(data):
        matches = [
            (match.group(), match.start(), match.end() - 1)
            for match in re.finditer(pattern, row)
        ]
        for number_info in matches:
            surrounding = []
            print(number_info)

            ### check if can go up
            if i - 1 >= 0:
                for num in range(number_info[1], number_info[2] + 1):
                    surrounding.append((i - 1, num))

                # up and left
                if number_info[1] - 1 >= 0:
                    surrounding.append((i - 1, number_info[1] - 1))

                # up and right
                if number_info[2] + 1 < x_max:
                    surrounding.append((i - 1, number_info[2] + 1))

            ### Check if can go down
            if i + 1 < y_max:
                for num in range(number_info[1], number_info[2] + 1):
                    surrounding.append((i + 1, num))

                # down and left
                if number_info[1] - 1 >= 0:
                    surrounding.append((i + 1, number_info[1] - 1))

                # down and right
                if number_info[2] + 1 < x_max:
                    surrounding.append((i + 1, number_info[2] + 1))

            ### Check left
            if number_info[1] - 1 >= 0:
                surrounding.append((i, number_info[1] - 1))

            ### Check right
            if number_info[2] + 1 < x_max:
                surrounding.append((i, number_info[2] + 1))

            ### Checking to see if there is a symbol in surrounding
            all_punctuation = [punct for punct in string.punctuation if punct != "."]

            for surround in surrounding:
                for punctuation in all_punctuation:
                    if data[surround[0]][surround[1]] == punctuation:
                        total_sum += int(number_info[0])

    print(total_sum)
    return total_sum


def c2(data: list) -> int:

    x_max = len(data[0])
    y_max = len(data)

    numbers_with_stars = {}

    pattern = r"\d{1,3}"

    for i, row in enumerate(data):
        matches = [
            (match.group(), match.start(), match.end() - 1)
            for match in re.finditer(pattern, row)
        ]
        for number_info in matches:
            surrounding = []
            # print(number_info)

            ### check if can go up
            if i - 1 >= 0:
                for num in range(number_info[1], number_info[2] + 1):
                    surrounding.append((i - 1, num))

                # up and left
                if number_info[1] - 1 >= 0:
                    surrounding.append((i - 1, number_info[1] - 1))

                # up and right
                if number_info[2] + 1 < x_max:
                    surrounding.append((i - 1, number_info[2] + 1))

            ### Check if can go down
            if i + 1 < y_max:
                for num in range(number_info[1], number_info[2] + 1):
                    surrounding.append((i + 1, num))

                # down and left
                if number_info[1] - 1 >= 0:
                    surrounding.append((i + 1, number_info[1] - 1))

                # down and right
                if number_info[2] + 1 < x_max:
                    surrounding.append((i + 1, number_info[2] + 1))

            ### Check left
            if number_info[1] - 1 >= 0:
                surrounding.append((i, number_info[1] - 1))

            ### Check right
            if number_info[2] + 1 < x_max:
                surrounding.append((i, number_info[2] + 1))

            ### Checking to see if there is a "*" in surrounding.
            ### If "*" found in surrounding, append to dictionary with key of
            ### coords with "*" and value of the number that is at the center
            for surround in surrounding:
                if data[surround[0]][surround[1]] == "*":
                    key_ = f"{surround[0]}_{surround[1]}"
                    if key_ not in numbers_with_stars:
                        numbers_with_stars[key_] = []

                    numbers_with_stars[key_].append(int(number_info[0]))

    # Add the product of coords with two numbers adjacent to them to get the solution
    solution = 0
    for v in numbers_with_stars.values():
        if len(v) == 2:
            solution += v[0] * v[1]

    print(solution)
    return solution


if __name__ == "__main__":
    data = read_data("input.txt")
    # print((data))
    c2(data)
