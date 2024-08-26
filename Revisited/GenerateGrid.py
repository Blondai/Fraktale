from numpy import linspace, meshgrid, float16, singlecomplex, zeros, array


def GenerateGrid(num_points_real: int = 1000, lower_real: float = -1, upper_real: float = 1,
                 num_points_complex: int = 1000, lower_complex: float = -1, upper_complex: float = 1) -> (array, array, array):
    real = linspace(lower_real, upper_real, num=num_points_real, endpoint=True, dtype=float16)
    complex = linspace(lower_complex, upper_complex, num=num_points_complex, endpoint=True, dtype=float16)
    real_v, complex_v = meshgrid(real, complex * 1.0j)
    grid = zeros((num_points_real, num_points_complex), dtype=singlecomplex)
    grid += real_v - complex_v
    return grid, real, complex
