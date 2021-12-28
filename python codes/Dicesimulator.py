import random
print("\nRolling Dice Program :\nRoll the dice please :\n")
x = "y"
while (x == 'y'):
    rnd = random.randint(1,6)
    if (rnd == 1):
        print("-----------")
        print("|         |")
        print("|    0    |")
        print("|         |")
        print("-----------")
    if (rnd == 2):
        print("-----------")
        print("|         |")
        print("| 0     0 |")
        print("|         |")
        print("-----------")
    if (rnd == 3):
        print("-----------")
        print("|    0    |")
        print("|    0    |")
        print("|    0    |")
        print("-----------")
    if (rnd == 4):
        print("-----------")
        print("| 0     0 |")
        print("|         |")
        print("| 0     0 |")
        print("-----------")
    if (rnd == 5):
        print("-----------")
        print("| 0     0 |")
        print("|    0    |")
        print("| 0     0 |")
        print("-----------")
    if (rnd == 6):
        print("-----------")
        print("| 0     0 |")
        print("| 0     0 |")
        print("| 0     0 |")
        print("-----------")
    x = input("Do you want to repeat: y or n ")
    if (x == "y" or x == "Y"):
        continue
    else:
        break
