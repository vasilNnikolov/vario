import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

if __name__ == "__main__":
    df = pd.read_csv("open-vario/log_file.csv", index_col=False)
    print(df.head())

    fig, ax = plt.subplots(nrows=5)
    df.plot(x="t", y="p", ax=ax[0])
    df.plot(x="t", y="v", ax=ax[1])
    df.plot(x="t", y="h", ax=ax[2])
    df.plot(x="t", y="filtered_v", ax=ax[3])
    df.plot(x="t", y="filted_h", ax=ax[4])
    plt.show()
