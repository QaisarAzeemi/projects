import random
import datetime
d = datetime.datetime.now()
print (d)
x = 5
y = 6
z = int (input ("enter the value of z "))

if (z > x):
    print('z is greater than x')
elif (z < x):
    print("z is less than x")
else:
    print("z is equal to x")
if (z > y):
    print("z is greater than y")
elif (z < y):
    print("z is less than y")
else:
    print("z is equal to y")
rnd = random.randint(1,27)
print('__________________________________')
print("z = ", z)
print("random number =", rnd)
print("Mix # = ", z+rnd)
print(' ')

file = open("test.txt", "r+")
x = file.read()
print(x)

fo = open("foo.txt", "a+")
fo.write("\n Accessing and writing the file in dev mode\n")
# fo.close()
y = fo.read()
print(y)

e = [[1,2,3], [4,5,6],[7,8,9]]
print(e)
print("\n")

import numpy as np
f = np.array(e)
print(f)

import matplotlib.pyplot as plt
#from cmath import log
#from math import pow
#matplotlib inline
e = np.linspace(0,10,100)
g = e ** 2
plt.plot(e,g)
#a = f"x = {x} and y = {y}"
#print(a)
#b = "z = {} and y = {}".format(z , y)
#print (b)

import matplotlib.pyplot as plt
from cmath import log
from math import pow

test_data = list(filter(
                lambda x: x != 0,
                [n / 100.0 for n in range(-1000, 1001)]))

equations = [
     (lambda x: 1),
     (lambda x: x),
     (lambda x: pow(x, 2)),
     (lambda x: pow(x, 3)),
     (lambda x: pow(x, 4)),
     (lambda x: pow(x, 7)),
     (lambda x: log(x).real)
]
plt.title("Test plot")
plt.xlabel('Time (x)')
plt.ylabel('Space (y)')
plt.axhline(0, color='black')
plt.axvline(0, color='black')
plt.xlim((-10, 10))
plt.ylim((-10, 10))

for equation in equations:
    plt.plot(test_data, list(map(equation, test_data)))

plt.savefig("out.png")
plt.show()
