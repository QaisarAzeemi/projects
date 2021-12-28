import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
x = np.linspace(0,10,20)
y = x * x
z = x+y
plt.title("Test plot")
plt.xlabel('Time (x)')
plt.ylabel('Space (y)')
plt.axhline(0, color='blue')
plt.axvline(0, color='blue')
plt.plot(x,y, label="time")
plt.legend(loc = 10)
plt.show()
plt.hist(x,y)
plt.show()
plt.fill(x,y)
plt.show()
plt.bar(x,y)
plt.show()
plt.polar(x,y)
plt.show()


# Fixing random state for reproducibility
np.random.seed(19680801)

dt = 0.01
t = np.arange(0, 30, dt)
nse1 = np.random.randn(len(t))                 # white noise 1
nse2 = np.random.randn(len(t))                 # white noise 2

# Two signals with a coherent part at 10Hz and a random part
s1 = np.sin(2 * np.pi * 10 * t) + nse1
s2 = np.sin(2 * np.pi * 10 * t) + nse2

fig, axs = plt.subplots(2, 1)
axs[0].plot(t, s1, t, s2)
axs[0].set_xlim(0, 2)
axs[0].set_xlabel('time')
axs[0].set_ylabel('s1 and s2')
axs[0].grid(True)

cxy, f = axs[1].cohere(s1, s2, 256, 1. / dt)
axs[1].set_ylabel('coherence')

fig.tight_layout()
plt.show()
