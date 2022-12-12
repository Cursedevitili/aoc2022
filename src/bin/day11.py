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
    def __init__(self, starting_items: list[int], operation: callable, test: callable, limit_worry: bool, demo_group: bool):
        self.starting_items = starting_items
        self.operation = operation
        self.test = test
        self.times_inspected = 0
        self.limit_worry = limit_worry
        self.demo_group = demo_group
        self.super_modulo_demo = 23 * 19 * 13 * 17
        self.super_modulo_real = 3 * 17 * 2 * 19 * 11 * 5 * 13 * 7

    def get_first_item(self) -> int:
        return self.starting_items.pop(0)

    def add_item(self, item):
        self.starting_items.append(item)

    def do_operation(self, item: int) -> int:
        new_item_level = self.operation(item)
        return new_item_level

    def inspect(self, item: int) -> (int, int):
        item_after_op = self.do_operation(item)
        if self.limit_worry:
            new_worry_level = int(item_after_op / 3)
        else:
            new_worry_level = item_after_op
        monkey_to_throw = self.test(new_worry_level)
        if not self.limit_worry:
            if self.demo_group:
                new_worry_level = new_worry_level % self.super_modulo_demo
            else:
                new_worry_level = new_worry_level % self.super_modulo_real
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
    monkey0demo = Monkey([79, 98], lambda x: x * 19, monkey0demo_test, True, True)
    monkey1demo = Monkey([54, 65, 75, 74], lambda x: x + 6, monkey1demo_test, True, True)
    monkey2demo = Monkey([79, 60, 97], lambda x: x * x, monkey2demo_test, True, True)
    monkey3demo = Monkey([74], lambda x: x + 3, monkey3demo_test, True, True)
    demo_group = Monkeys([monkey0demo, monkey1demo, monkey2demo, monkey3demo])

    monkey0demoWorry = Monkey([79, 98], lambda x: x * 19, monkey0demo_test, False, True)
    monkey1demoWorry = Monkey([54, 65, 75, 74], lambda x: x + 6, monkey1demo_test, False, True)
    monkey2demoWorry = Monkey([79, 60, 97], lambda x: x * x, monkey2demo_test, False, True)
    monkey3demoWorry = Monkey([74], lambda x: x + 3, monkey3demo_test, False, True)
    demo_groupWorry = Monkeys([monkey0demoWorry, monkey1demoWorry, monkey2demoWorry, monkey3demoWorry])

    monkey0 = Monkey([66, 71, 94], lambda x: x * 5, monkey0_test, True, False)
    monkey1 = Monkey([70], lambda x: x + 6, monkey1_test, True, False)
    monkey2 = Monkey([62, 68, 56, 65, 94, 78], lambda x: x + 5, monkey2_test, True, False)
    monkey3 = Monkey([89, 94, 94, 67], lambda x: x + 2, monkey3_test, True, False)
    monkey4 = Monkey([71, 61, 73, 65, 98, 98, 63], lambda x: x * 7, monkey4_test, True, False)
    monkey5 = Monkey([55, 62, 68, 61, 60], lambda x: x + 7, monkey5_test, True, False)
    monkey6 = Monkey([93, 91, 69, 64, 72, 89, 50, 71], lambda x: x + 1, monkey6_test, True, False)
    monkey7 = Monkey([76, 50], lambda x: x * x, monkey7_test, True, False)
    real_group = Monkeys([monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7])

    monkey0worry = Monkey([66, 71, 94], lambda x: x * 5, monkey0_test, False, False)
    monkey1worry = Monkey([70], lambda x: x + 6, monkey1_test, False, False)
    monkey2worry = Monkey([62, 68, 56, 65, 94, 78], lambda x: x + 5, monkey2_test, False, False)
    monkey3worry = Monkey([89, 94, 94, 67], lambda x: x + 2, monkey3_test, False, False)
    monkey4worry = Monkey([71, 61, 73, 65, 98, 98, 63], lambda x: x * 7, monkey4_test, False, False)
    monkey5worry = Monkey([55, 62, 68, 61, 60], lambda x: x + 7, monkey5_test, False, False)
    monkey6worry = Monkey([93, 91, 69, 64, 72, 89, 50, 71], lambda x: x + 1, monkey6_test, False, False)
    monkey7worry = Monkey([76, 50], lambda x: x * x, monkey7_test, False, False)
    real_group_worry = Monkeys([monkey0worry, monkey1worry, monkey2worry, monkey3worry, monkey4worry, monkey5worry, monkey6worry, monkey7worry])

    for _ in range(20):
        demo_group.inspect_items()
        demo_group.print_state()
    demo_group.print_monkey_business_level()
    print()

    for _ in range(20):
        real_group.inspect_items()
        real_group.print_state()
    real_group.print_monkey_business_level()
    print()

    for i in range(1, 10001):
        demo_groupWorry.inspect_items()
        # demo_groupWorry.print_state()
        if i == 1 or i == 1000 or i == 2000 or i == 10000:
            demo_groupWorry.print_monkey_business_level()

    print()
    print("Real")
    for i in range(1, 10001):
        real_group_worry.inspect_items()
        # demo_groupWorry.print_state()
        if i == 1 or i == 1000 or i == 2000 or i == 10000:
            real_group_worry.print_monkey_business_level()


if __name__ == "__main__":
    main()
