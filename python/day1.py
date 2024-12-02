import sys
from pathlib import Path

def input():
    list1, list2 = [], []
    file = Path(sys.argv[1])
    content = file.read_text()
    for line in content.splitlines():
        values = [x.strip() for x in line.split() if x.strip()]
        list1.append(int(values[0]))
        list2.append(int(values[1]))
    return (list1, list2)

def part_one():
    list1, list2 = input()
    
    tot_distance = 0
    while (len(list1) > 0 and len(list2) > 0):
        min1 = min(list1)
        min2 = min(list2)
        tot_distance += abs(min1 - min2)
        list1.remove(min1)
        list2.remove(min2)
    print(f"Total distance is {tot_distance}")

def part_one_v2():
    list1, list2 = input()
    
    list1.sort()
    list2.sort()
    
    tot_distance  = sum(abs(x-y) for x,y in zip(list1, list2))
    print(f"Total distance is {tot_distance}")


if __name__ == "__main__":
    part_one_v2()