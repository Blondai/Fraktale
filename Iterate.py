from numpy import abs, logical_or, logical_and, where, int8, zeros, array


def Iterate(grid: array, fun, limit: float = 2, iterations: int = 100) -> array:
    brightness = zeros(grid.shape)

    for n in range(iterations):
        indicator = logical_or(abs(grid.real) > limit, abs(grid.imag) > limit)
        brightness += (1 - n / iterations) * int8(logical_and(indicator, brightness == 0))
        grid = where(indicator, grid, fun(grid))
    return brightness
