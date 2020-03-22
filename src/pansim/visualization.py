from matplotlib import pyplot as plt
import pandas as pd
import sys

df = pd.read_csv(sys.argv[1])
df.plot()
plt.show()
