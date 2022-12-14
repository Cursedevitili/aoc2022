def comp_arr(a, b) -> int:
    if type(a) == list and type(b) == int:
        b = [b]
    if type(b) == list and type(a) == int:
        a = [a]
    if type(a) == int and type(b) == int:
        if a < b:
            return 1
        elif a > b:
            return -1
    if type(a) == list and type(b) == list:
        i = 0
        while i < len(a) and i < len(b):
            inner = comp_arr(a[i], b[i])
            if inner == 1:
                return 1
            elif inner == -1:
                return -1
            i = i + 1
        if len(a) == len(b):
            return 0
        elif len(a) < len(b):
            return 1
        else:
            return -1


def main():
    with open("../../input/day13.txt") as f:
        lines = f.readlines()
    arrs = []
    for i in range(0, len(lines), 3):
        row1 = eval(lines[i])
        row2 = eval(lines[i + 1])
        arrs.append(row1)
        arrs.append(row2)

    count = 1
    right_pairs = []
    for i in range(0, len(arrs), 2):
        res = comp_arr(arrs[i], arrs[i + 1])
        if res == 1:
            right_pairs.append(count)
        count = count + 1

    print(right_pairs)
    print(sum(right_pairs))


if __name__ == "__main__":
    main()
