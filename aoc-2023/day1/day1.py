def read_data(path_):
    with open(path_, "r") as f:
        lines = [line.strip() for line in f.readlines()]
    return lines


def challenge1(data: list) -> int:
    line_digits = []
    for line in data:
        all_digits = [char for char in line if char.isdigit()]
        if len(all_digits) >= 2:
            line_digits.append(int(all_digits[0] + all_digits[-1]))
        if len(all_digits) == 1:
            line_digits.append(int(all_digits[0] * 2))

    return sum(line_digits)


def recursive_replace(in_string: str) -> str:
    if "one" in in_string:
        in_string = in_string.replace("one", "o1e")
        return recursive_replace(in_string)
    elif "two" in in_string:
        in_string = in_string.replace("two", "t2")
        return recursive_replace(in_string)
    elif "three" in in_string:
        in_string = in_string.replace("three", "t3e")
        return recursive_replace(in_string)
    elif "four" in in_string:
        in_string = in_string.replace("four", "4")
        return recursive_replace(in_string)
    elif "five" in in_string:
        in_string = in_string.replace("five", "5e")
        return recursive_replace(in_string)
    elif "six" in in_string:
        in_string = in_string.replace("six", "6")
        return recursive_replace(in_string)
    elif "seven" in in_string:
        in_string = in_string.replace("seven", "7n")
        return recursive_replace(in_string)
    elif "eight" in in_string:
        in_string = in_string.replace("eight", "e8t")
        return recursive_replace(in_string)
    elif "nine" in in_string:
        in_string = in_string.replace("nine", "n9e")
        return recursive_replace(in_string)
    return in_string

def recursive_replace2(in_string: str) -> str:
    replacement_dict = {
        "one": "o1e",
        "two": "t2",
        "three": "t3e",
        "four": "4",
        "five": "5e",
        "six": "6",
        "seven": "7n",
        "eight": "e8t",
        "nine": "n9e",
    }

    for k, v in replacement_dict.items():
        if k in in_string:
            in_string = in_string.replace(k, v)
            return recursive_replace2(in_string)
    return in_string

def challenge2(data: list) -> int:
    line_digits = []

    for line in data:
        digitized_line = recursive_replace2(line)

        all_digits = [char for char in digitized_line if char.isdigit()]

        line_digits.append(int(all_digits[0] + all_digits[-1]))

    return sum(line_digits)


if __name__ == "__main__":

    data = read_data("day1_input.txt")

    print(challenge2(data))
