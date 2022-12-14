from operator import itemgetter

def get_vals(parse_string: str):
    ref_arr: list
    arr = []
    ref_arr = [arr]
    i = 0
    val_str = ""
    for c in parse_string:
        if c == "[":
            new_ref = []
            ref_arr[i].append(new_ref)
            ref_arr.append(new_ref)
            i = i + 1
        if c == "]":
            if val_str != "":
                val = int(val_str)
                ref_arr[i].append(val)
                val_str = ""
            ref_arr.pop()
            i = i - 1
        if c.isnumeric():
            val_str = val_str + c
        if c == "," and val_str != "":
            val = int(val_str)
            ref_arr[i].append(val)
            val_str = ""
    if val_str != "":
        ref_arr[0].append(int(val_str))

    return arr


iter_refs_left = []
iter_refs_right = []

class AsdList():
    def __init__(self, s: list):
        self.ref: list[list] = [s]
        self.refC: list[int] = [0]
        self.s = s

    def get_next(self):
        retval = None
        if len(self.ref[-1]) > self.refC[-1]:
            retval = self.ref[-1][self.refC[-1]]
            self.refC[-1] = self.refC[-1] + 1
            if type(retval) == list:
                self.ref.append(retval)
                self.refC.append(0)
        elif len(self.ref) > 1 and len(self.ref[-2]) >= self.refC[-2]:
            self.ref.pop()
            self.refC.pop()
            retval = self.get_next()

        return retval


def key_cust(item):
    if type(item) == int:
        ret = [item]
    elif type(item[0]) == list:
        ret = item[0]
    else:
        ret = item
    return ret


def comp_pair(left_list: list, right_list: list ) -> (bool, bool):
    print(f"New comp")
    print(left_list)
    print(right_list)
    left = AsdList(left_list)
    right = AsdList(right_list)
    lv = -1
    rv = -1
    while lv != None or rv != None:
        if lv == -1:
            lv = left.get_next()
        if rv == -1:
            rv = right.get_next()
        print(f"Left op: {lv} Right op {rv}")
        if type(lv) != type(rv) and (lv != None and rv != None): # Diff types and not none
            if type(lv) == list and type(rv) == int and lv != []:
                lv = -1
                continue
            elif type(lv) == int and type(rv) == list and rv != []:
                rv = -1
                continue
            elif type(lv) == int and rv == []:
                print(f"False")
                return False
            elif lv == [] and type(rv) == int:
                print(f"True")
                return True
        elif type(lv) == type(rv) and type(lv) == list:
            if lv == rv and lv == []:
                lv = -1
                rv = -1
            elif lv == []:
                print(f"True")
                return True
            elif rv == []:
                print(f"False")
                return False
            else:
                lv = -1
                rv = -1
        elif type(lv) == type(rv) and type(lv) == int: # Both ints
            if lv != rv:
                print(f"{lv < rv}")
                return lv < rv
            else:
                lv = -1
                rv = -1
        elif (lv != None or rv != None) and lv != rv:
            if lv == None:
                print(f"True")
                return True
            else:
                print(f"False")
                return False

    return True




def main():
    with open("../../input/day13.txt") as f:
        lines = f.readlines()
    vals = []
    itertor = iter(lines)
    while 1:
        left = next(itertor)
        # print(left)
        vals.append(get_vals(left[1:-1]))
        right = next(itertor)
        # print(right)
        vals.append(get_vals(right[1:-1]))
        stop = next(itertor, None)
        if stop == None:
            break

    right_pairs = []
    count = 1
    for i in range(0, len(vals), 2):
        print(f"Round {count}")
        res = comp_pair(vals[i], vals[i+1])
        if res:
            right_pairs.append(count)
        count = count + 1

    print(right_pairs)
    print(f"Sum of these is: {sum(right_pairs)}")


if __name__ == "__main__":
    main()
