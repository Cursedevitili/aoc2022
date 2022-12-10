import re as regex

class Prog:
    def __init__(self):
        self.x = 1
        self.cycles = 1
        self.log = [(self.cycles, self.x, self.x * self.cycles,0)]

    def noop(self):
        self.cycles = self.cycles + 1
        self.log.append((self.cycles, self.x, self.x * self.cycles, 0))

    def add(self, val):
        self.cycles = self.cycles + 1
        self.log.append((self.cycles, self.x, self.x * self.cycles, 0))
        self.cycles = self.cycles + 1
        self.x = self.x + val
        self.log.append((self.cycles, self.x, self.x * self.cycles, val))

    def get_sum_of(self, which: list):
        sum_of_which = 0
        for value in which:
            log = self.log[value-1]
            temp = log[2]
            sum_of_which = sum_of_which + temp
        return sum_of_which
    def get_sum_total(self):
        return sum(map(lambda x: x[2], self.log))

    def __str__(self):
        str = ""
        for entry in self.log:
            str = str + f"cycle: {entry[0]} x: {entry[1]} signal strength: {entry[2]}\n"
        return str

def main():
    with open("./../../input/day10demo.txt") as f:
        lines: input = f.readlines()

    prog = Prog()

    add_pat = "addx (-*[0-9]+)"
    noop_pat = "noop"
    count = 1
    for line in lines:
        count = count + 1
        add_match = regex.match(add_pat, line)
        noop_match = regex.match(noop_pat, line)
        if add_match != None:
            value = int(add_match.group(1))
            prog.add(value)
        elif noop_match != None:
            prog.noop()

    print(prog)

    print(f"Sum is {prog.get_sum_of([20, 60, 100, 140, 180, 220])}")



if __name__ == "__main__":
    main()
