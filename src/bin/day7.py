# Can't be done in rust with my skills
import re
from typing import List, Any


class File:
    def __init__(self, name: str, size: int):
        self.name = name
        self.size = size
class Folder:
    def __init__(self, name, level: int):
        self.name: str = name
        self.files: [File] = []
        self.folders: [Folder] = []
        self.upper: Folder = None
        self.level = level
        self.size_if_deleted: int = None

    def add_file(self, name, size):
        self.files.append(File(name, size))

    def add_folder(self, name, level):
        self.folders.append(Folder(name, level))

    def move_folder_up(self, name):
        fold: Folder
        for fold in self.folders:
            if fold.name == name:
                return fold

        raise Exception

    def add_upper(self, folder: "Folder"):
        self.upper = folder

    def find_files_sum_recurring(self, accumulator):
        files_sum = sum(list(map(lambda n: n.size, self.files)))
        folder: Folder
        for folder in self.folders:
            files_sum += folder.find_files_sum_recurring(files_sum)
        return files_sum

    def recur_find_folders_under_100_000(self, start: int, total_size: int, folders: list["Folder"]) -> (int, int, list["Folder"]):
        files_sum = sum(list(map(lambda n: n.size, self.files)))
        size_deeper = 0
        for folder in self.folders:
            size_deeper += folder.find_files_sum_recurring(size_deeper)

        self.size_if_deleted = files_sum + size_deeper
        folders.append(self)

        total_size = total_size
        # print(f"Dir {self.name} files sum: {files_sum} + inderect: {size_deeper}, sum is {files_sum+size_deeper}")
        if files_sum + size_deeper <= 100000:
            start += 1
            total_size += files_sum + size_deeper

        fold: Folder
        for fold in self.folders:
            (start, total_size) = fold.recur_find_folders_under_100_000(start, total_size, folders)

        return start, total_size

    def __str__(self):
        tab = "\t"
        outstring = ""
        for fold in self.folders:
            outstring += f"{tab * self.level}Dir {fold.name}\n"
            outstring += str(fold)

        f:File
        for f in self.files:
            outstring += f"{tab * self.level}File {f.name} {f.size}\n"

        return outstring

def main():
    file_structure = Folder("root", 0)
    current: Folder = file_structure
    with open("./../../input/day7.txt") as f:
        lines = f.readlines()
        # lines.next()
        lines: str
        line_counter = 1;
        for line in lines:
            goto_dir = re.match("\\$ cd ([a-zA-Z]+)", line)
            back = re.match("\\$ cd \.\.", line)
            new_file = re.match("([0-9]+).([a-zA-Z]+.*[a-zA-Z]*)", line)
            new_folder = re.match("dir ([a-zA-Z]+)", line)
            if "$ ls" in line:
                pass
            if new_file:
                # print(line)
                # print(f"name match: {new_file.group(2)} size {new_file.group(1)}")
                current.add_file(new_file.group(2), int(new_file.group(1)))
            elif new_folder:
                # print(f"new folder match: {new_folder.group(1)}")
                current.add_folder(new_folder.group(1), current.level+1)
            elif back:
                if current.upper != None:
                    current = current.upper
            elif goto_dir:
                # print(f"gotodir match: {goto_dir.group(1)}")
                temp = current
                current = current.move_folder_up(goto_dir.group(1))
                current.add_upper(temp)
            else:
                pass
            line_counter += 1

    print(file_structure)
    folder_list = []
    result, total_size = file_structure.recur_find_folders_under_100_000(0, 0, folder_list)
    out1 = f"Folder sizes under 100000: {result}, summed size: {total_size}"
    print(out1)

    def sort_func(e: Folder):
        return e.size_if_deleted
    folder_list.sort(key=sort_func)

    files_sum = sum(list(map(lambda n: n.size_if_deleted, folder_list)))
    total_free_disk_space = 70000000 - file_structure.size_if_deleted
    still_needed = 30000000 - total_free_disk_space
    done = False
    for folder in folder_list:
        if folder.size_if_deleted >= still_needed and not done:
            out2 = f"Name: {folder.name} size if deleted: {folder.size_if_deleted} is enough"
            print(out2)
            done = True

    with open('./../../output/day7output.txt', 'w') as f:
        f.write(out1+"\n")
        f.write(out2)



if __name__ == "__main__":
    main()