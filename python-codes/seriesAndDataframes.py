import numpy as np
import pandas as pd 
A = [1,2,3,4]
B = [5,6,7,8]
C = [9,10,11,12]
D = ['I', 'II', 'III', 'IV']
E = ['a','b','c','d']
df = pd.DataFrame([A,B,C,D,E],['a', 'b','c','d','e'],['W','X','Y','Z'])
df['A'] = df['Y'] + df['Z'] #adding two columns
print(df)
x = pd.Series(A,D)
print(x)

y = df.drop( labels= 'd', axis=0) # d is not perminantly deleted
print(y)
print(df)

y = df.drop( labels= 'Y', axis=1) # Y is not perminantly deleted
print("y = \n" , y)
print(df)

x = df.drop('Y', axis=1, inplace = True) # Y is perminantly deleted
print("x = \n", x)
print(df)
#print(df[df['A'] > 3])

#Using PANDAS to analyze and use data

fruit_seller = { 'fruits': ['Apple', 'Apple', 'Orange', 'Orange', 'Water Malons', 'Water Malons'], 'days': ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday','Saturday'], 'amount (kg)':[100,140, 85,105, 20,30]}
print(fruit_seller)
y = pd.DataFrame(fruit_seller)
print(y)
y = y.groupby('fruits')
print('\n      Group = \n', y)
mean =y.mean()
print('\n      mean sales = \n', mean)
total =y.sum()
print('\n      Sum = \n', total)
count =y.count()
print('\n      count = \n', count)
maximum =y.max()
print('\n      Maximum = \n', maximum)
minimum =y.min()
print('\n      Minimum = \n', minimum )
describe = y.describe()
print('\n      Describe = \n', describe)

print('_______________JOINING_______________')
#-----------JOINING---------------------
x1={'a': [1,2,3,4], 'b':[5,6,7,8]}
y1={'c':[1,3,5,7], 'd':[2,4,6,8]}
print ('x1 = \n', pd.DataFrame(x1))
print ('y1 = \n', pd.DataFrame(y1))
x = pd.DataFrame(x1, index = ['I','II','III','IV'])
y = pd.DataFrame(y1, index = ['I','II','III','IV'])
z = x.join(y)
print(z)
z = y.join(x, how="inner")
print('\n')
print(z)

#__________________________CONCATENATING_________________________________
print('________________________CONCATENATING_________________________________')
x1={'key1': [10,11,12,13], 'a': [1,2,3,4], 'b':[5,6,7,8]}
y1={'key1': [10,11,12,13], 'c':[1,3,5,7], 'd':[2,4,6,8]}
print(pd.DataFrame(x1))
print(pd.DataFrame(y1))
x = pd.DataFrame(x1)
y = pd.DataFrame(y1)
z = pd.merge(y,x, how='right', on = 'key1')
print(z)
