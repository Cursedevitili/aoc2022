import re as regex

class Prog:
    def __init__(self):
        self.x = 1
        self.cycles = 1
        self.log = [(self.cycles, self.x, self.x * self.cycles)]
        self.sprite_pos = 0
        self.image = ""

    def move_sprite(self, val: int):
        self.sprite_pos = self.sprite_pos + val

    def is_sprite_and_pixel_in_same_position(self):
        row_position = (self.cycles % 40)-1
        if row_position == 0 and self.cycles != 0:
            self.image = self.image + "\n"

        spritemax = self.sprite_pos + 2
        if self.sprite_pos <= row_position <= spritemax:
            self.image = self.image + "#"
        else:
            self.image = self.image + "."


    def print_sprite_state(self):
        print(f"Cycle: {self.cycles}")
        print("Image")
        print(self.image)
        sprite_visualizer = ""
        for i in range(40):
            if self.sprite_pos <= i <= (self.sprite_pos+2):
                sprite_visualizer = sprite_visualizer + "#"
            else:
                sprite_visualizer = sprite_visualizer + "."
        print("Sprite")
        print(sprite_visualizer)
        print()

    def noop(self):
        self.is_sprite_and_pixel_in_same_position()
        self.print_sprite_state()
        self.cycles = self.cycles + 1
        self.log.append((self.cycles, self.x, self.x * self.cycles))

    def add(self, val):
        self.is_sprite_and_pixel_in_same_position()
        self.print_sprite_state()
        self.cycles = self.cycles + 1
        self.log.append((self.cycles, self.x, self.x * self.cycles))
        self.is_sprite_and_pixel_in_same_position()
        self.print_sprite_state()
        self.cycles = self.cycles + 1
        self.x = self.x + val
        self.log.append((self.cycles, self.x, self.x * self.cycles))
        self.move_sprite(val)

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
    with open("./../../input/day10.txt") as f:
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
    out1 = f"Sum is {prog.get_sum_of([20, 60, 100, 140, 180, 220])}"
    print(out1)
    out2 = f"CRT image: \n{prog.image}"
    print(out2)

    with open('./../../output/day10output.txt', 'w') as f:
        f.write(out1+"\n\n")
        f.write(out2)



if __name__ == "__main__":
    main()
