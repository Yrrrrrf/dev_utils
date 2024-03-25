# python packages
from typing import Callable
# external imports
from sympy import Symbol, lambdify
# own imports

import pathlib  # import with pathlib
path = pathlib.Path(__file__).parent.parent.parent.parent.absolute()  # get the path of the current file
print(f"path: {path}")
import sys  # import sys
sys.path.append(str(path))  # add the path of the current file to the sys.path list
# from math.util.functions import get_derivative, get_g_function  # relative import
from scripts.math.util.functions import get_derivative, get_g_function  # absolute import


# ? ONE VARIABLE NUMERICAL ANALYSIS METHODS ------------------------------------------------------------------------------------------------

'''
One variable numerical analysis methods.

Closed methods:
    - bisection method  (2.1)
    - false rule method  (2.5)
Open methods:
    - fixed point method (2.2)
    - secant method (2.4)
    - newton-raphson method (2.3)

# note: the newton raphson method use a derivative function, so it's necessary to import sympy and use the sympy library to create the derivative function.

# todo: pending implementaions:
    Open methods:
        - fixed point method (2.2)
    - Steffensen's method (2.6)
    - Horner's method (2.7)
    - Muller's method (2.8)
'''


# ? CLOSED METHODS ----------------------------------------------------------------------------------------------------------------
# This methods consist in bisecting the interval [a, b] until the root is found to within the tolerance.


def bisection_method(f: Callable[[float], float], a: float, b: float, tol=1e-6, max_iter=100) -> float:
    """
    Iteratively bisect the interval [a, b] until the root is found to within the tolerance.

    :param f: function
    :param a: left bound
    :param b: right bound
    :param tol: tolerance
    :param max_iter: maximum iteration
    :return: root
    """
    if f(a) * f(b) > 0: raise ValueError(f"f(a)^f(b) must have different signs")
    c = (a + b) / 2  # (just for avoiding the error: UnboundLocalError: local variable 'c' referenced before assignment)
    for _ in range(max_iter):
        c = (a + b) / 2  # find the midpoint (bisect the interval)
        if f(c) == 0 or (b - a) / 2 < tol: return c
        if f(a) * f(c) < 0: b = c  # if f(a) and f(c) have different signs, then the root is in the interval [a, c]
        else:  a = c  # otherwise, the root is in the interval [c, b]
    return c


def false_rule(f: Callable[[float], float], a:float, b:float, tol=1e-6, max_iter=100) -> float:
# Is really similar to the secant method, but it uses the midpoint of the interval [a, b] instead of the secant line.
    """
    Iteratively approximate the root using (a*f(b) - b*f(a)) / (f(b) - f(a)) until the root is found to within the tolerance.
    This method consist in bisecting the interval [a, b] until the root is found to within the tolerance.
    :param f: function
    :param a: left bound
    :param b: right bound
    :param tol: tolerance
    :param max_iter: maximum iteration
    :return: root
    """
    if f(a) * f(b) > 0: raise ValueError(f"f(a)^f(b) must have different signs")
    c = b - ((f(b) * (b - a)) / (f(b) - f(a)))  # (just for avoiding the error: UnboundLocalError: local variable 'c' referenced before assignment)
    for _ in range(max_iter):
        # c = (a*f(b) - b*f(a)) / (f(b) - f(a))  is equal to c = b - ((f(b) * (b - a)) / (f(b) - f(a))) so IT'S THE SAME
        c = b - ((f(b) * (b - a)) / (f(b) - f(a)))  # find the midpoint (approximate the root)
        if f(c) == 0 or (b - a) / 2 < tol: return c
        if f(a) * f(c) < 0: b = c  # if f(a) and f(c) have different signs, then the root is in the interval [a, c]
        else:  a = c  # otherwise, the root is in the interval [c, b]
    return c


# ? OPEN METHODS ------------------------------------------------------------------------------------------------------------------
# This methods consist in finding the root by using the tangent line to the function f(x) at the point x0
# An open method could not converge to the root, so it's necessary to set a maximum number of iterations.

def secant_method(f: Callable[[float], float], x0: float, x1: float, tol=1e-6, max_iter=100) -> float:
# Is really similar to the false rule method, but is USES THE LAST TWO APPROXIMATIONS TO FIND THE NEXT ONE.
    """
    Iteraitvely evaluate f(x) to find a closer approximation of the root. 
    Use x_new = x - f(x) * (x - x_old) / (f(x) - f(x_old)) to find the next approximation.

    Parameters
    ----
    :param f: function
    :param x0: initial guess
    :param x1: initial guess
    :param tol: tolerance
    :param max_iter: maximum iteration
    :return: root
    """
    x = x0  # initial guess
    xnew = x1  # (just for avoiding the error: UnboundLocalError: local variable 'xnew' referenced before assignment)
    for _ in range(max_iter):
        xnew = x - f(x) * (x - x1) / (f(x) - f(x1))  # next iteration
        if abs(xnew - x) < tol: return xnew  # if the difference between the two iterations is less than the tolerance, then the root is found
        x1 = x  # update the value of x1 (the previous value of x)
        x = xnew  # update the value of x (the current value of x)
    return xnew


def newton_raphson_method(f: Callable[[float], float], x0: float, tol=1e-6, max_iter=100) -> float:
    """
    Iteratively evaluate f(x) and f'(x) to find a closer approximation of the root.
    Use x_new = x - f(x) / f'(x) to find the next approximation.
    :param f: function
    :param x0: initial guess
    :param tol: tolerance
    :param max_iter: maximum iteration
    :return: root
    """
    x = x0  # initial guess
    xnew = x0  # initial guess (just for avoiding the error: UnboundLocalError: local variable 'xnew' referenced before assignment)
    diff = get_derivative(f)
    for _ in range(max_iter):
        xnew = x - f(x) / diff(x)  # next iteration
        if abs(xnew - x) < tol: return float(xnew)  # if the difference between the two iterations is less than the tolerance, then the root is found
        x = xnew  # update the value of x
    return float(xnew)  # return the root in a float format


# todo: fixed-point method
def fixed_point_method(f: Callable[[float], float], x0: float, tol=1e-6, max_iter=100) -> float:
    """
    Iteratively evaluate f(x) to find a closer approximation of the root.
    Use x_new = f(x) to find the next approximation.
    :param f: function
    :param x0: initial guess
    :param tol: tolerance
    :param max_iter: maximum iteration
    :return: root
    """
    x = x0
    xnew = x0
    f = get_g_function(f)
    for _ in range(max_iter):
        xnew = f(x)  # next iteration # wtf??
        if abs(xnew - x) < tol: return xnew  # if the difference between the two iterations is less than the tolerance, then the root is found
        x = xnew  # update the value of x
    return xnew


if __name__ == '__main__':

    x = Symbol('x', real=True)
    f = x**3 - 2*x - 5
    # f = x**2 - 2
    # f = x**2 + 4
    f = lambdify(x, f, 'numpy')  # Convert the sympy expression to a numpy function -> f(x) = x**3 - 2*x - 5

    a, b = 1, 4  # interval

    # ? open methods
    print(f'Bisection method: {bisection_method(f, a, b)}')
    print(f'False rule method: {false_rule(f, a, b)}')
    # ? closed methods
    print(f'Secant method: {secant_method(f, a, b)}')   
    print(f'Newton-Raphson method: {newton_raphson_method(f, 2)}')
    # ! test
    # print(f'Fixed-point method: {fixed_point_method(f, a)}')


    # ? Plot the function
    # import matplotlib.pyplot as plt
    # x = np.linspace(1, 3, 100)  # 100 linearly spaced numbers
    # plt.plot(x, f(x))  # (x, y)
    # plt.grid()
    # plt.show()
