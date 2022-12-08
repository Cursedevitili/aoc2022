class Forest:
    def __init__(self, forest_input: list):
        temp_root = []
        line: str
        for line in forest_input:
            temp = []
            for char in line:
                if char != "\n":
                    temp.append(int(char))
            temp_root.append(temp)

        self.contents: [[int]] = temp_root

    def get_highest_top(self, i, j, acc) -> int:
        res = acc
        if i > 0:
            if res < self.contents[i-1][j]:
                res = self.contents[i-1][j]
            res = self.get_highest_top(i - 1, j, res)
        return res

    def get_highest_low(self, i, j, acc):
        res = acc
        if i < len(self.contents)-1:
            if res < self.contents[i+1][j]:
                res = self.contents[i+1][j]
            res = self.get_highest_low(i + 1, j, res)
        return res

    def get_highest_left(self, i, j, acc):
        res = acc
        if j > 0:
            if res < self.contents[i][j-1]:
                res = self.contents[i][j-1]
            res = self.get_highest_left(i, j-1, res)
        return res

    def get_highest_right(self, i, j, acc):
        res = acc
        if j < len(self.contents[i])-1:
            if res < self.contents[i][j+1]:
                res = self.contents[i][j+1]
            res = self.get_highest_right(i, j+1, res)
        return res


    def get_how_many_not_visible(self):
        count = 0
        lim_i = len(self.contents)-1
        lim_j = len(self.contents[0])-1
        for i in range(1, lim_i):
            for j in range(1, lim_j):
                curr_value = self.contents[i][j]
                top = self.get_highest_top(i, j, 0)
                left = self.get_highest_left(i, j, 0)
                right = self.get_highest_right(i, j, 0)
                low = self.get_highest_low(i, j, 0)
                if (top >= curr_value) \
                        and (left >= curr_value) \
                        and (right >= curr_value) \
                        and (low >= curr_value):
                    count = count + 1

        return count


    def get_total_trees(self):
        return len(self.contents)*len(self.contents[0])
def main():
    with open("./../../input/day8.txt") as f:
        lines: list = f.readlines()

    forest = Forest(lines)
    not_visible = forest.get_how_many_not_visible()
    total = forest.get_total_trees()
    print(f"Visible trees: {total - not_visible}")


if __name__ == "__main__":
    main()