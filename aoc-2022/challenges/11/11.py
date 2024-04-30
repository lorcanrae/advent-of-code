TEST_PRODUCT = 2 * 7 * 13 * 3 * 19 * 5 * 11 * 17


class Monkey:

    def __init__(self, number, items, operation, test, if_true, if_false):
        self.number = number
        self.items = items
        self.operation = operation
        self.test = test
        self.if_true = if_true
        self.if_false = if_false
        self.inspects = 0

    def inspect_item(self, item, relief):
        self.inspects += 1
        worry = eval(self.operation.replace("old", item))
        if relief == 1:
            worry = worry % TEST_PRODUCT
        else:
            worry = worry // relief
        self.items.remove(item)
        if worry % self.test == 0:
            return self.if_true, str(worry)
        else:
            return self.if_false, str(worry)


def get_data():
    with open("input.txt", "r") as f:
        lines = [line.strip("\n") for line in f]

    monkeys_raw = []
    for i in range(0, len(lines), 7):
        monkeys_raw.append([line.strip() for line in lines[i : i + 6]])

    monkeys = []
    for monkey in monkeys_raw:
        monkey_num = int(monkey[0].split()[-1].strip(":"))
        monkey_items = [item.strip(",") for item in monkey[1].split()[2:]]
        monkey_operation = monkey[2].split(" = ")[1]
        monkey_test = int(monkey[3].split()[-1])
        monkey_if_true = int(monkey[4].split()[-1])
        monkey_if_false = int(monkey[5].split()[-1])
        monkeys.append(
            Monkey(
                monkey_num,
                monkey_items,
                monkey_operation,
                monkey_test,
                monkey_if_true,
                monkey_if_false,
            )
        )

    return monkeys


def part1(monkeys):
    for i in range(20):
        for monkey in monkeys:
            while monkey.items:
                for item in monkey.items:
                    throw_to, worry = monkey.inspect_item(item, 3)
                    monkeys[throw_to].items.append(worry)

    top_two = sorted([monkey.inspects for monkey in monkeys], reverse=True)[:2]
    monkey_biz = top_two[0] * top_two[1]
    print(monkey_biz)

    return monkey_biz


def part2(monkeys):
    for i in range(10_000):
        for monkey in monkeys:
            while monkey.items:
                for item in monkey.items:
                    throw_to, worry = monkey.inspect_item(item, 1)
                    monkeys[throw_to].items.append(worry)

    top_two = sorted([monkey.inspects for monkey in monkeys], reverse=True)[:2]
    monkey_biz = top_two[0] * top_two[1]
    print(monkey_biz)

    return monkey_biz


if __name__ == "__main__":
    part1(get_data())
    part2(get_data())
