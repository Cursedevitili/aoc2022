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
        self.not_count_loc = []
        self.max_i = len(self.contents)
        self.max_j = len(self.contents[0])
        self.max_score = (0,0,0)

    def get_highest_top(self, i, j, acc) -> int:
        res = acc
        if i > 0:
            if res < self.contents[i - 1][j]:
                res = self.contents[i - 1][j]
            res = self.get_highest_top(i - 1, j, res)
        return res

    def get_highest_low(self, i, j, acc):
        res = acc
        if i < len(self.contents) - 1:
            if res < self.contents[i + 1][j]:
                res = self.contents[i + 1][j]
            res = self.get_highest_low(i + 1, j, res)
        return res

    def get_highest_left(self, i, j, acc):
        res = acc
        if j > 0:
            if res < self.contents[i][j - 1]:
                res = self.contents[i][j - 1]
            res = self.get_highest_left(i, j - 1, res)
        return res

    def get_highest_right(self, i, j, acc):
        res = acc
        if j < len(self.contents[i]) - 1:
            if res < self.contents[i][j + 1]:
                res = self.contents[i][j + 1]
            res = self.get_highest_right(i, j + 1, res)
        return res

    def get_how_many_not_visible(self):
        count = 0
        lim_i = len(self.contents) - 1
        lim_j = len(self.contents[0]) - 1
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
                    self.not_count_loc.append((i, j))

        return count

    def get_range_top(self, i, j, start_val, dist) -> int:
        res = dist
        if i > 0 and start_val > self.contents[i - 1][j]:
            res = dist + 1
            res = self.get_range_top(i - 1, j, start_val, res)
        elif i > 0 and start_val <= self.contents[i - 1][j]:
            res = dist + 1
        return res

    def get_range_low(self, i, j, start_val, dist):
        res = dist
        if i < (len(self.contents) - 1) and start_val > self.contents[i + 1][j]:
            res = dist + 1
            res = self.get_range_low(i + 1, j, start_val, res)
        elif i < (len(self.contents)- 1) and start_val <= self.contents[i + 1][j]:
            res = dist + 1
        return res

    def get_range_left(self, i, j, start_val, dist):
        res = dist
        if j > 0 and start_val > self.contents[i][j - 1]:
            res = dist + 1
            res = self.get_range_left(i, j - 1, start_val, res)
        elif j > 0 and start_val <= self.contents[i][j - 1]:
            res = dist + 1
        return res

    def get_range_right(self, i, j, start_val, dist):
        res = dist
        if j < (len(self.contents[i]) - 1) and start_val > self.contents[i][j + 1]:
            res = dist + 1
            res = self.get_range_right(i, j + 1, start_val, res)
        elif j < (len(self.contents[i]) - 1) and start_val <= self.contents[i][j + 1]:
            res = dist + 1
        return res

    def calc_scenic_value(self, x, y):
        top = self.get_range_top(x, y, self.contents[x][y], 0)
        if top == 0:
            top = 1
        left = self.get_range_left(x, y, self.contents[x][y], 0)
        if left == 0:
            left = 1
        right = self.get_range_right(x, y, self.contents[x][y], 0)
        if right == 0:
            right = 1
        low = self.get_range_low(x, y, self.contents[x][y], 0)
        if low == 0:
            low = 1
        return top * left * right * low

    def get_scenic_values(self):
        row: list
        for i in range(1, self.max_i-1):
            for j in range(1, self.max_j-1):
                comp_obj = (i, j)
                if not comp_obj in self.not_count_loc:
                    result = self.calc_scenic_value(i, j)
                    if self.max_score[2] < result:
                        self.max_score = (i, j, result)


    def get_total_trees(self):
        return len(self.contents) * len(self.contents[0])


def main():
    with open("./../../input/day8.txt") as f:
        lines: list = f.readlines()

    forest = Forest(lines)
    not_visible = forest.get_how_many_not_visible()
    total = forest.get_total_trees()
    out1 = f"Visible trees: {total - not_visible}"
    print(out1)
    forest.get_scenic_values()
    out2 = f"Max scenic score: {forest.max_score[2]} in " \
           f"pos x: {forest.max_score[0]} " \
           f"y:{forest.max_score[1]}"
    print(out2)

    with open('./../../output/day8output.txt', 'w') as f:
        f.write(out1+"\n")
        f.write(out2)


if __name__ == "__main__":
    main()
