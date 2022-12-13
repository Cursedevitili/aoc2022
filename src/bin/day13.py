from typing import Optional

class Arr:
    def __init__(self, level, upper):
        self.vals = []
        self.level = level
        self.upper: Arr = upper
        self.count = 0

    def add_value(self, val):
        self.vals.append(val)

    def add_arr(self, new):
        self.vals.append(new)

    def get_upper(self) -> "Arr":
        return self.upper

    def get_next(self):
        if self.count < len(self.vals):
            ret_val = self.vals[self.count]
            if isinstance(ret_val, Arr):
                ret_val = ret_val.get_next()
            else:
                self.count = self.count + 1
            return ret_val
        elif self.upper != None:
            self.upper.count = self.upper.count + 1
            return self.upper.get_next()
        else:
            return None

    def get_deepest(self):
        if len(self.vals) > 0:
            retval = self.vals[0]
            if isinstance(retval, Arr):
                return retval.get_deepest()
        else:
            return self.level


class Comparator:
    def __init__(self, left: str, right: str):
        self.left_string: str = left[1:-1]
        self.right_string: str = right[1:-1]
        self.left = Arr(0, None)
        self.right = Arr(0, None)
        self.count_left = 0

    def comp_next(self):
        pass

    def get_vals(self, use_left = True):
        if use_left:
            parse_string = self.left_string
            curr = self.left
        else:
            parse_string = self.right_string
            curr = self.right
        val_str = ""
        for c in parse_string:
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

    def compare_pair(self) -> bool:
        comp_round = 0
        while 1:
            comp_round = comp_round + 1
            comp_left = self.left.get_next()
            comp_right = self.right.get_next()
            if comp_left == None and comp_right == None and comp_round == 1:
                left = self.left.get_deepest()
                right = self.right.get_deepest()
                return left < right

            if comp_left == None or comp_right == None:
                if comp_left == None and comp_right == None:
                    return True
                return comp_left == None
            if type(comp_left) == int and type(comp_right) == int:
                if comp_left == comp_right:
                    continue
                else:
                    return comp_left < comp_right



def main():
    with open("../../input/day13demo.txt") as f:
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

    right_pairs = []
    for i, pair in enumerate(comps):
        pair.get_vals(use_left=True)
        pair.get_vals(use_left=False)
        result = pair.compare_pair()
        if result:
            right_pairs.append(i+1)

    print(right_pairs)
    print(f"Sum of these is: {sum(right_pairs)}")



if __name__ == "__main__":
    main()
