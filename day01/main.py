import re

filename = "input.txt"
with open(filename) as f:
    content = f.readlines()

r = re.compile('(?=(one|two|three|four|five|six|seven|eight|nine|zero|\d))')
#r = re.compile('(?=(\d))')
d = {
        "one":'1',
        "two":'2',
        "three":'3',
        "four":'4',
        "five":'5',
        "six":'6',
        "seven":'7',
        "eight":'8',
        "nine":'9',
        "zero":'0',
        }
res = 0
for line in content:
    finds = r.findall(line)
    if (len(finds[0]) == 1):
        first = finds[0]
    else:
        first = d[finds[0]]

    last_index = len(finds)-1
    if (len(finds[last_index]) == 1):
        last = finds[last_index]
    else:
        last = d[finds[last_index]]
    res += int(first+last)

print("res : ", res)


