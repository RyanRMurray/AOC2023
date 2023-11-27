import os

for i in range(2,26):
    os.system("cp day01.rs day{:02d}.rs".format(i))
    os.system("sed -i 's/day01/day{:02d}/g' day{:02d}.rs".format(i,i))
    os.system("sed -i 's/Day01/Day{:02d}/g' day{:02d}.rs".format(i,i))