def monkey0demo_test(val: int):
    if val % 23 == 0:
        return 2
    else:
        return 3


def monkey1demo_test(val: int):
    if val % 19 == 0:
        return 2
    else:
        return 0


def monkey2demo_test(val: int):
    if val % 13 == 0:
        return 1
    else:
        return 3


def monkey3demo_test(val: int):
    if val % 17 == 0:
        return 0
    else:
        return 1


def monkey0_test(val: int):
    if val % 3 == 0:
        return 7
    else:
        return 4


def monkey1_test(val: int):
    if val % 17 == 0:
        return 3
    else:
        return 0


def monkey2_test(val: int):
    if val % 2 == 0:
        return 3
    else:
        return 1


def monkey3_test(val: int):
    if val % 19 == 0:
        return 7
    else:
        return 0


def monkey4_test(val: int):
    if val % 11 == 0:
        return 5
    else:
        return 6


def monkey5_test(val: int):
    if val % 5 == 0:
        return 2
    else:
        return 1


def monkey6_test(val: int):
    if val % 13 == 0:
        return 5
    else:
        return 2


def monkey7_test(val: int):
    if val % 7 == 0:
        return 4
    else:
        return 6


class Monkey:
    def __init__(self, starting_items: list[int], operation: callable, test: callable):
        self.starting_items = starting_items
        self.operation = operation
        self.test = test
        self.times_inspected = 0

    def get_first_item(self) -> int:
        return self.starting_items.pop(0)

    def add_item(self, item):
        self.starting_items.append(item)

    def do_operation(self, item: int) -> int:
        new_item_level = self.operation(item)
        return new_item_level

    def inspect(self, item: int) -> (int, int):
        item_after_op = self.do_operation(item)
        new_worry_level = int(item_after_op / 3)
        monkey_to_throw = self.test(new_worry_level)
        self.times_inspected = self.times_inspected + 1
        return monkey_to_throw, new_worry_level


class Monkeys:
    def __init__(self, monkeys: list[Monkey]):
        self.monkeys = monkeys
        self.round = 0

    def add_item_to_monkey(self, which: int, item: int):
        self.monkeys[which].add_item(item)

    def inspect_items(self):
        self.round = self.round + 1
        for monkey in self.monkeys:
            for _ in range(len(monkey.starting_items)):
                item_to_test = monkey.get_first_item()
                which_monkey, item_with_new_level = monkey.inspect(item_to_test)
                self.add_item_to_monkey(which_monkey, item_with_new_level)

    def print_state(self):
        print(f"Round {self.round}")
        for i in range(len(self.monkeys)):
            print(f"Monkey {i}: {self.monkeys[i].starting_items}")
        print()

    def print_monkey_business_level(self):
        print()
        levels = []
        for i, monkey in enumerate(self.monkeys):
            levels.append(monkey.times_inspected)
            print(f"Monkey {i} inspected items {monkey.times_inspected}")
        levels.sort(reverse=True)
        print(f"Level of monkey business {levels[0] * levels[1]}")


def main():
    monkey0demo = Monkey([79, 98], lambda x: x * 19, monkey0demo_test)
    monkey1demo = Monkey([54, 65, 75, 74], lambda x: x + 6, monkey1demo_test)
    monkey2demo = Monkey([79, 60, 97], lambda x: x * x, monkey2demo_test)
    monkey3demo = Monkey([74], lambda x: x + 3, monkey3demo_test)
    demo_group = Monkeys([monkey0demo, monkey1demo, monkey2demo, monkey3demo])

    monkey0 = Monkey([66, 71, 94], lambda x: x * 5, monkey0_test)
    monkey1 = Monkey([70], lambda x: x + 6, monkey1_test)
    monkey2 = Monkey([62, 68, 56, 65, 94, 78], lambda x: x + 5, monkey2_test)
    monkey3 = Monkey([89, 94, 94, 67], lambda x: x + 2, monkey3_test)
    monkey4 = Monkey([71, 61, 73, 65, 98, 98, 63], lambda x: x * 7, monkey4_test)
    monkey5 = Monkey([55, 62, 68, 61, 60], lambda x: x + 7, monkey5_test)
    monkey6 = Monkey([93, 91, 69, 64, 72, 89, 50, 71], lambda x: x + 1, monkey6_test)
    monkey7 = Monkey([76, 50], lambda x: x * x, monkey7_test)
    real_group = Monkeys([monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7])

    for _ in range(20):
        demo_group.inspect_items()
        demo_group.print_state()
    demo_group.print_monkey_business_level()

    print()
    print("Real")
    for _ in range(20):
        real_group.inspect_items()
        real_group.print_state()
    real_group.print_monkey_business_level()


if __name__ == "__main__":
    main()
