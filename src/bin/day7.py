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

    def add_file(self, name, size):
        self.files.append(File(name, size))

    def add_folder(self, name, level):
        self.folders.append(Folder(name, level))

    def move_folder_up(self, name):
        fold: Folder
        for fold in self.folders:
            if fold.name == name:
                return fold

    def add_upper(self, folder: "Folder"):
        self.upper = folder

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
    current = file_structure
    last = file_structure
    with open("./../../input/day7demo.txt") as f:
        lines = f.readlines()
        # lines.next()
        lines: str
        for line in lines:
            goto_dir = re.findall("\\$ cd ([a-zA-Z])", line)
            back = re.findall("\\$ cd ..", line)
            new_file = re.findall("([0-9]+) ([a-zA-Z]+.*[a-zA-Z]*)", line)
            new_folder = re.findall("dir ([a-zA-Z]+)", line)
            if "$ ls" in line:
                pass
            if len(new_file) > 0:
                current.add_file(new_file[0][1], new_file[0][0])
            elif len(new_folder) > 0:
                current.add_folder(new_folder[0], current.level+1)
            elif len(back) > 0:
                current = current.upper
            elif len(goto_dir) > 0:
                temp = current
                current = current.move_folder_up(goto_dir[0][0])
                current.add_upper(temp)

    print(file_structure)

if __name__ == "__main__":
    main()