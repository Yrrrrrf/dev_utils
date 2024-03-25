from typing import Callable

import numpy as np


'''
Approximation
Is the process of finding a function that is close to a given function in some sense.

Least Squares Regression Methods:
    - Linear Regression
    - Polynomial Regression (Multiple Linear Regression)
    - Exponential Regression
    - Power Regression (simple)
'''


def linear_regression(x, y) -> Callable:
    """
    Interpolate the data (x, y)

    :param x: the x values
    :param y: the y values
    :return: the linear regression function in the form y = mx + b
    """
    # ! return np.poly1d(np.polyfit(x, y, 1))  # but we won't use this because wi'll implement it ourselves
    x_mean = np.mean(x)
    y_mean = np.mean(y)
    # cov = np.mean(x * y) - x_mean * y_mean  # covariance
    # m = cov / np.var(x)  # slope
    m = (np.mean(x * y) - x_mean * y_mean) / np.var(x)  # slope
    b = y_mean - m * x_mean  # y-intercept
    print(f'y = {m}x + {b}')
    return lambda x: m * x + b


def exponential_regression(x, y) -> Callable:
    """
    Interpolate the data (x, y)

    :param x: the x values
    :param y: the y values
    :return: the exponential regression function in the form y = a * e^(bx)
    """
    # ! return np.exp(np.polyfit(np.log(x), y, 1, full=True))  # but we won't use this because wi'll implement it ourselves
    y = np.log(y)  # we need to take the log of 'y' because we want to find the slope of the log of 'y'
    x_mean = np.mean(x)
    y_mean = np.mean(y)
    m = (np.mean(x * y) - x_mean * y_mean) / np.var(x)  # slope
    b = y_mean - m * x_mean  # y-intercept
    print(f'y = {np.exp(b)}e^{m}x')
    return lambda x: np.exp(m * x + b)


def power_regression(x, y) -> Callable:
    """
    Interpolate the data (x, y)

    :param x: the x values
    :param y: the y values
    :return: the power regression function
    """
    # ! return np.poly1d(np.polyfit(np.log(x), np.log(y), 1, full=True))  # but we won't use this because wi'll implement it ourselves
    x = np.log(x)
    y = np.log(y)
    x_mean = np.mean(x)
    y_mean = np.mean(y)
    # cov = np.mean(x * y) - x_mean * y_mean  # covariance
    # m = cov / np.var(x)  # slope
    m = (np.mean(x * y) - x_mean * y_mean) / np.var(x)  # slope
    b = y_mean - m * x_mean  # y-intercept
    print(f'y = {np.exp(b)}x^{m}')
    return lambda x: np.exp(m * np.log(x) + b)


def polynomial_regression(x, y, degree: int = 2) -> Callable:
    """
    Also known as Curve Fitting process
    Interpolate the data (x, y)

    :param x: the x values
    :param y: the y values
    :return: the polynomial regression function in the form y = ax^2 + bx + c
    """
    # ! return np.poly1d(np.polyfit(x, y, degree))  # but we won't use this because wi'll implement it ourselves
    A = np.zeros((degree + 1, degree + 1))  # degree+1 square matrix
    for row in range(degree + 1):
        for column in range(degree + 1): 
            A[row][column] = np.sum(x ** (row + column))
    # print(f'A = {A}')
    b = np.zeros(degree + 1)  # degree+1 column vector (results)
    for row in range(degree + 1):
        b[row] = np.sum((x ** row) * y)
    # print(f'b = {b}')
    x = np.linalg.solve(A, b)  # A * x = b => x = A^-1 * b
    # print(f'x = {x}')
    print(f'y = ', end='')
    for i in range(degree + 1):
        print(f'{x[i]:.4f}x^{i}', end='') if i == degree else print(f'{x[i]:.4f}x^{i} + ', end='')
        # print(f'{x[i]:}x^{i}', end='') if i == degree else print(f'{x[i]:}x^{i} + ', end='')
    print()  # new line
    return lambda x: np.sum([x ** i * x[i] for i in range(degree + 1)])


if __name__ == '__main__':
    # Create some data.
    # x = np.linspace(0, 10, 10)
    # y = np.linspace(1, 21, 11)
    x = np.arange(1, 6)
    y = np.array([0.5, 1.7, 3.4, 5.7, 8.4])

    print(f'Linear Regression: {linear_regression(x, y)}')
    print(f'Polynomial Regression: {polynomial_regression(x, y, degree=10)}')
    print(f'Exponential Regression: {exponential_regression(x, y)}')
    print(f'Power Regression: {power_regression(x, y)}')
