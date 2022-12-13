class Arr:
    def __init__(self, level, upper):
        self.vals = []
        self.level = level
        self.upper = upper

    def add_value(self, val):
        self.vals.append(val)
    def add_arr(self, new):
        self.vals.append(new)

    def get_upper(self) -> "Arr":
        return self.upper

class Comparator:
    def __init__(self, left: str, right: str):
        self.left_string: str = left[1:-1]
        self.right_string: str = right[1:-1]
        self.left = Arr(0, None)
        self.right = [0]
        self.count_left = 0

    def comp_next(self):
        pass

    def get_vals_left(self):
        curr = self.left
        val_str = ""
        for c in self.left_string:
            if c == "[":
                new = Arr(curr.level+1, curr)
                curr.add_arr(new)
                curr = new
            if c == "]":
                if val_str != "":
                    val = int(val_str)
                    curr.add_value(val)
                    val_str = ""
                curr = curr.get_upper()
            if c.isnumeric():
                val_str = val_str+c
            if c == "," and val_str != "":
                val = int(val_str)
                curr.add_value(val)
                val_str = ""


def main():
    with open("./../../input/day13.txt") as f:
        lines = f.readlines()

    itertor = iter(lines)
    comps: [Comparator] = []
    while 1:
        left = next(itertor)
        print(left)
        right = next(itertor)
        print(right)
        comps.append(Comparator(left, right))
        stop = next(itertor, None)
        if stop == None:
            break

    for pair in comps:
        pair.get_vals_left()



if __name__ == "__main__":
    main()
