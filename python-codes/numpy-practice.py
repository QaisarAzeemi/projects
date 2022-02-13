import numpy as np
x = [1,2,3],[4,5,6],[7,8,9]
y = ['a','b','c']
print("x = ",x)
print(y)
z1 = np.array(x)
z2 = np.array(y)
print(z1)
print(z2)
print("______________NUMPY METHODS________________________________")

#NUMPY METHODS
x = np.zeros((3,3))-9 #zero matrix - 9
print(x)

x = np.ones((3,3))-9 #one matrix - 9
print(x)

x = np.eye(5)  #unity matrix
print(x)

x = np.arange(10) #range of numbers
print(x)

x = np.arange(70).reshape(10,7) #10 rows and 7 columns
print(x)

x = np.linspace(10,3,6) #6 numbers between 10 and 3
print(x)

z3 = x.max() #maximum value
print(z3)

z3 = x.min()
print(z3)

x = np.random.rand(5) # 5 random numbers between 0 and 1
print(x)

x = np.random.randint(500) #a random number between 0 and 500
print(x)
print("_______________SLICING AND INDEXING_______________________________")

#SLICING AND INDEXING
x = np.arange(1, 20)
print(x)

print (x[4]) #element at 4th index
print (x[-1]) #last element
print (x[:10]) #from 0 to 9th index
print (x[10:]) #from 10th to last index

y = x[4:7] = 255 #replacing index 4 to index 6 with 255
print(x)

y = x.copy() #copying the array
print(y)

print("____________2D SLICING AND INDEXING__________________________________")

# 2D SLICING AND INDEXING
x = np.arange(25).reshape(5,5)
print(x)

y = x [1:3, 1:4] #slicing from 1 to 2 rows and 1 to 3 columns
print (y)

y = np.arange(16) #creating a 16 element array
print (y)
print (y>4) #boolean array
print (y[y>10]) 

print("_____________OPERATIONS IN NUMPY__________________________________")

# OPERATIONS IN NUMPY

x = np.arange(10)
y = np.arange(11,21)
print(x)
print(y)
print(x+y)
print(np.sin(x))
print(np.tan(x))
print(np.log(x))
print(np.reciprocal(x))
print(np.sum(x))
print(np.exp(x))
print(np.median(x))
print(np.std(x)) #full population standard dev
print(np.full((3,3),'T'))

a = np.array([1,1,2,3,4,5,6])

b = np.array([0,2,1,3,4,8,9])
print(np.intersect1d(a,b))
print(a[(a>1)&(a<=4)])
