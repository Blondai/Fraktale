import matplotlib.pyplot as plt
from numpy import min, max, array


def Visualize(brightness: array, real: array, complex: array):
    axis = (min(real), max(real), min(complex), max(complex))
    fig, ax = plt.subplots()
    ax.imshow(brightness, cmap="gray", vmin=0, vmax=1, extent=axis)
    ax.get_xaxis().set_visible(False)
    ax.get_yaxis().set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.spines['left'].set_visible(False)
    ax.spines['top'].set_visible(False)
    ax.spines['bottom'].set_visible(False)
    return fig, ax
