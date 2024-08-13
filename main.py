from GenerateGrid import GenerateGrid
from Iterate import Iterate
from Visualize import Visualize

import numpy as np
import matplotlib.pyplot as plt

fun = np.poly1d([0.3, 1, 0, complex(-0.1, 0.61)])

if __name__ == "__main__":
    grid, real, complex = GenerateGrid(lower_real=-1.4, upper_real=1.1, lower_complex=-1.5, upper_complex=1, num_points_real=1000, num_points_complex=1000)
    brightness = Iterate(grid, fun, iterations=100)
    fig, ax = Visualize(brightness, real, complex)
    plt.show()