from matplotlib import pyplot as plt
import pandas as pd
import sys
import os

path = sys.argv[1]
if os.path.isfile(path):
    p = path
    if not p.endswith('.csv'):
        sys.exit(1)
    p = '.'.join(p.split('.')[:-1])
    df = pd.read_csv(sys.argv[1])
    df.plot()
    plt.savefig(p + '.pdf')
    plt.savefig(p + '.png')
elif os.path.isdir(path):
    for p in os.listdir(path):
        if not p.endswith('.csv'):
            continue
        p = '.'.join(p.split('.')[:-1])
        print(p)
        df = pd.read_csv(p)
        #plt.set_yscale('log')
        df.plot().set_yscale('log')
        plt.savefig(p + '.pdf')
        plt.savefig(p + '.png')
