from typing import Callable
from sympy.abc import x
import sympy as sp


'''
Is the process of finding the area under a curve.

Integration Methods (Newton-Cotes Integration):
    - Closed Methods:
        - Trapezoidal (composite) (at least 2 points)
        - Simpson 1/3 (composite) (at least 3 points)
        - Simpson 3/8 (composite) (at least 4 points)
        - Boole (composite) (at least 5 points)
    - Open Methods:
        - Midpoint (at least 2 points)
        - Two-point (at least 2 points)
        - n-point (is an abstraction of the previous two methods) (at least 2 points)
    - Gaussian Quadrature(at least 2 points)


# todo: pending implementaions:
    - Composite Simpson's Rule (4.1)
    - Romberg Integration (4.2)
    - Adaptive Quadrature (4.3) (an evolutioon of Gaussian Quadrature)
    - Simpson's Double Integral (4.4)
    - Guassian Double Integral (4.5)
    - Guassian Triple Integral (4.6)
'''


# ? Closed Methods ----------------------------------------------------------------------------------------------------------------


def trapezoidal_rule(f: Callable[[float], float], a: float, b:float, n: int) -> float:
    '''
    Trapezoidal Method
    :param f: function
    :param a: lower bound
    :param b: upper bound
    :param n: number of intervals
    '''
    step = (b - a) / n  # interval size (step size)
    s = 0.5 * (f(a) + f(b))  # sum of all other terms
    s += sum(f(a + i * step) for i in range(1, n))  # sum of all other terms
    return s * step  # multiply by interval size. So the first and last terms are multiplied by 1/2. The rest of the terms are multiplied by 1.    


def simpson_1_3_method(f: Callable[[float], float], a: float, b:float, n: int) -> float:
    '''
    Simpson's 1/3 Method
    :param f: function
    :param a: lower bound
    :param b: upper bound
    :param n: number of intervals
    '''
    h = (b - a) / n  # interval size (step size)
    s = f(a) + f(b)  # first and last terms
    for i in range(1, n):
        s += 2 * f(a + i * h) if i % 2 == 0 else 4 * f(a + i * h)  # add 2*f(x) if i is even, else add 4*f(x)
        # if it's not even it means that is odd, so it's multiplied by 4
    return s * h / 3  # multiply by interval size and divide by 3. So the first and last terms are multiplied by 1. The rest of the terms are multiplied by 2 or 4.


def simpson_3_8_method(f: Callable[[float], float], a: float, b:float, n: int) -> float:
    '''
    Simpson's 3/8 Method
    :param f: function
    :param a: lower bound
    :param b: upper bound
    :param n: number of intervals
    '''
    h = (b - a) / n  # interval size (step size)
    s = f(a) + f(b)  # first and last terms
    for i in range(1, n):
        s += 2 * f(a + i * h) if i % 3 == 0 else 3 * f(a + i * h)  # add 2*f(x) if i is multiple of 3, else add 3*f(x)
    return 3 * h * s / 8  # multiply by interval size and divide by 8. So the first and last terms are multiplied by 1. The rest of the terms are multiplied by 2 or 3.


def boole_method(f: Callable[[float], float], a: float, b:float, n: int) -> float:
    '''
    Boole's Method
    :param f: function
    :param a: lower bound
    :param b: upper bound
    :param n: number of intervals
    '''
    h = (b - a) / n  # interval size (step size)
    s = 7 * (f(a) + f(b))  # first and last terms
    s += 32 * sum(f(a + i * h) for i in range(1, n, 2))  # odd terms
    s += 12 * sum(f(a + i * h) for i in range(2, n, 4))  # even terms that are not multiples of 4
    s += 14 * sum(f(a + i * h) for i in range(4, n, 4))  # even terms that are multiples of 4
    return (2 * h * s) / 45  # multiply by interval size and divide by 45. So the first and last terms are multiplied by 7. The rest of the terms are multiplied by 2 or 14.
 

# ? Open Methods ------------------------------------------------------------------------------------------------------------------


def midpoint_method(f: Callable[[float], float], a: float, b:float, n: int) -> float:
    '''
    Midpoint Method
    :param f: function
    :param a: lower bound
    :param b: upper bound
    :param n: number of intervals
    '''
    h = (b - a) / n  # interval size (step size)
    s = sum(f(a + (i * h) - (h / 2)) for i in range(1, n + 1))  # sum of all terms
    return s * h  # multiply by interval size
    # Returns s*h because the function is evaluated at the midpoint of the interval. So all terms are multiplied by 1.


def n_point_method(f: Callable[[float], float], a: float, b:float, n: int, k: int = 2) -> float:
    '''
    N-Point Method
    :param f: function
    :param a: lower bound
    :param b: upper bound
    :param n: number of intervals
    :param k: number of points
    '''
    h = (b - a) / n  # interval size (step size)
    s = 0
    for i in range(1, n + 1):
        for j in range(1, k + 1):
            s += f(a + i * h - h + h * (j - 1) / (k - 1))  # sum of all terms
    return s * h / k  # multiply by interval size and divide by k
    # Returns s*h/k because the function is evaluated at the k points of the interval. So all terms are multiplied by 1/k.


# ? Gaussian Quadrature -----------------------------------------------------------------------------------------------------------


# todo: implement this method
#! fix the returned functions (roots & cohefficients)
def gaussian_quadrature(f: Callable[[float], float], x: float, n: int = 2) -> Callable[[float], float]:
    '''
    Use Gaussian Quadrature to create a function that approximates the integral of f(x) with respect to x
    :param f: function
    :param x: variable
    :param n: number of points
    :return: function
    '''
    f = _gaussian_substitution(f)  # get the f(t) function
    if n == 2: return lambda x: (f(x) + f(x)) / 2
    if n == 3: return lambda x: (5 * f(x) + 8 * f(x) + 5 * f(x)) / 18
    if n == 4: return lambda x: (19 * f(x) + 25 * f(x) + 25 * f(x) + 19 * f(x)) / 36
    if n == 5: return lambda x: (41 * f(x) + 54 * f(x) + 59 * f(x) + 54 * f(x) + 41 * f(x)) / 90
    if n == 6: return lambda x: (751 * f(x) + 989 * f(x) + 1091 * f(x) + 1091 * f(x) + 989 * f(x) + 751 * f(x)) / 1800
    else: return lambda x: 0


# ? Util Funcitions ---------------------------------------------------------------------------------------------------------------
# def integrate(f: Callable[[float], float], limits: Tuple[Symbol, float, float]) -> float:
#     '''
#     Integrate f(x) with respect to x
#     :param f: function
#     :param limits: limits of integration
#     :return: integral
#     '''
#     return integrate(f, limits).evalf()


# ! fix this methhod
def _gaussian_substitution(f: Callable[[float], float]) -> Callable[[float], float]:
    """
    Is a function that takes a function f(x) and returns a function f(t) that is the result of substituting x with t
    This function is used in the gaussian quadrature method
    :param f: function
    :return: function
    """
    f = sp.sympify(f).subs(sp.Symbol('x'), sp.Symbol('t'))
    return sp.lambdify(sp.Symbol('t'), f, 'numpy')


if __name__ == '__main__':

    f = lambda x: 1 / (1 + x ** 2)
    # f = lambda x: math.exp(x**4)
    a, b = 0, 1
    # a, b = -1, 1
    # n = 1000000  # number of intervals
    n = 100

    print('Exact Value:', sp.integrate(1 / (1 + x ** 2), (x, a, b)), "==", float(sp.integrate(1 / (1 + x ** 2), (x, a, b)).evalf()))  # type: ignore

    print('Trapezoidal Rule:', trapezoidal_rule(f, a, b, n))
    print('Simpson\'s 1/3 Method:', simpson_1_3_method(f, a, b, n))
    print('Simpson\'s 3/8 Method:', simpson_3_8_method(f, a, b, n))
    print('Boole\'s Method:', boole_method(f, a, b, n))

    print('Midpoint Method:', midpoint_method(f, a, b, n))
    print('N-Point Method:', n_point_method(f, a, b, n, 2))

    # print('Gaussian Quadrature:', gaussian_quadrature(f, a, b, n))
