from typing import Callable


import scipy as sp
from sympy import Symbol, solve, lambdify


'''
Interpolation is the process of finding a function that passes through a set of points.
These methods use the Lagrange polynomial to find the interpolating polynomial.

# todo: pending implementaions:
Interpolation Methods:
    - Lagrange Interpolation (Lagrange Polynomial)
    - Neville's Iterative (3.1)
    - Newton's Divided Differences (3.2)
    - Hermite Interpolation (3.3)

# todo: pending implementaions:
    - Natural Cubic Spline (3.4)
    - Clamped Cubic Spline (3.5)
'''


def lagrange_interpolation(x: list[float], y: list[float]) ->  Callable[[float], float]:
    """
    Creates the Lagrange polynomial that interpolates the data (x, y) at the points xnew.
    :param x: x values
    :param y: y values
    :return: Lagrange polynomial (lambda function)
    """
    return sp.interpolate.lagrange(x, y)


def nevile_interpolation(x, y, xnew) -> float:
    """
    Interpolate the data (x, y) at the points xnew.
    """

    return 0


def divided_differences(x, y, xnew) -> float:
    """
    Interpolate the data (x, y) at the points xnew.
    """
    return 0


def hermite_interpolation(x, y, xnew) -> float:
    """
    Interpolate the data (x, y) at the points xnew.
    """
    return 0


# def natural_cubic_spline(x, y, xnew) -> float:
#     """
#     Interpolate the data (x, y) at the points xnew.
#     """
#     return 0


# def clamp_cubic_spline(x, y, xnew) -> float:
#     """
#     Interpolate the data (x, y) at the points xnew.
#     """
#     return 0


if __name__ == '__main__':
    pass

