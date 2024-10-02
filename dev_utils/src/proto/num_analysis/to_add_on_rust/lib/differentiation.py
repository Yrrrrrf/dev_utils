from typing import Callable

import numpy as np


'''
Is the process of finding the derivative of a function.
A derivative is a measure of the rate of change of a function.

Differentiation Methods:
    - Forward Difference (at least 2 points)
    - Backward Difference (at least 2 points)
    - Central Difference (at least 3 points)
    - Richardson Extrapolation (at least 2 points)
'''

def forward_difference(f: Callable[[float], float], x: float, h: float) -> float:
    '''
    Approximates the value of the derivative of f(x) using the Forward Difference Method

    Args:
        :param f: function
        :param x: point
        :param h: step size
    '''
    return (f(x + h) - f(x)) / h


def backward_difference(f: Callable[[float], float], x: float, h: float) -> float:
    '''
    Approximates the value of the derivative of f(x) using the Backward Difference Method
    :param f: function
    :param x: point
    :param h: step size
    '''
    return (f(x) - f(x - h)) / h


def central_difference(f: Callable[[float], float], x: float, h: float) -> float:
    '''
    Approximates the value of the derivative of f(x) using the Central Difference Method
    :param f: function
    :param x: point
    :param h: step size
    '''
    return (f(x + h) - f(x - h)) / (2 * h)


# ? Richardson Extrapolation ----------------------------------------------------------------------------------------------------------------


def richardson_extrapolation(f: Callable[[float], float], x: float, h: float) -> float:
    '''
    Approximates the value of the derivative of f(x) using the Richardson Extrapolation Method
    This method is used to improve the accuracy of the Central Difference Method
    :param f: function
    :param x: point
    :param h: step size
    '''
    return (4 * central_difference(f, x, h / 2) - central_difference(f, x, h)) / 3


if __name__ == '__main__':
    # f = lambda x: np.sin(x)
    f = lambda x: (x**3)*np.cos(x)
    x = 2  # point
    h = 0.2  # step size

    print(f'Forward Difference: {forward_difference(f, x, h)}')
    print(f'Backward Difference: {backward_difference(f, x, h)}')
    print(f'Central Difference: {central_difference(f, x, h)}')
    print(f'Richardson Extrapolation: {richardson_extrapolation(f, x, h)}')

    # plot the functions
    from matplotlib import pyplot as plt
    x = np.linspace(0, np.pi, 100)
    plt.plot(x, f(x))
    plt.show()

