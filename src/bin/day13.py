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

    return arr


iter_refs_left = []
iter_refs_right = []


def comp_pair(left_list: iter, right_list: iter, left_def = None, right_def = None ) -> (bool, bool):
    if left_def != None:
        left = left_def
    else:
        left = next(left_list, None)

    if right_def != None:
        right = left_def
    else:
        right = iter(right_list, None)
    while right != None or left != None:
        if type(left) == list or type(right) == list:
            comp_pair(left_list, right_list)
        elif type(left) == list and right_def != None:
            comp_pair(left, right)
        elif left_def != None and type(right) == list:
            comp_pair(left, right)
        elif type(left_list[i]) == int and type(right_list[i]) == int and left_list[i] != left_list[i]:
            return left_list[i] < left_list[i]
        elif left_def != None and right_def != None and left_def == right_def:
            return True




def main():
    with open("../../input/day13demo.txt") as f:
        lines = f.readlines()
    vals = [list]
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
    for i in range(0, len(vals), 2):
        pass
        # left = iter(vals[i])
        # right = iter(vals[i+1])
        # comp_pair(left, right)

    print(right_pairs)
    print(f"Sum of these is: {sum(right_pairs)}")


if __name__ == "__main__":
    main()
