import sympy
import numpy as np
from sympy import Symbol, solve, lambdify


'''
Linear Systems are a set of equations with the same variables.

# todo: pending implementaions:
Linear Systems Methods:
    - LU Factorization (6.4)
        - Doolittle Algorithm (Linear Algebra)
        - Choleski Method (6.6)
        - Crout Algorithm (6.7)

# todo: pending implementaions:
    - Gaussian Elimination (6.1)
    - Gaussian Elimination Partial Pivoting (6.2)
    - Gaussian Elimination Scaled Pivoting (6.3)
    - LDL Factorization (6.5)
'''


def lu_factorization(A, b):
    """
    Solve the linear system Ax = b using LU factorization.
    :param A: matrix
    :param b: vector
    :return: solution
    """
    n = len(A)
    for k in range(0, n - 1):
        for i in range(k + 1, n):
            if A[i, k] != 0.0:
                lam = A[i, k] / A[k, k]
                A[i, k + 1:n] = A[i, k + 1:n] - lam * A[k, k + 1:n]
                A[i, k] = lam
    for k in range(1, n):
        b[k] = b[k] - np.dot(A[k, 0:k], b[0:k])
    b[n - 1] = b[n - 1] / A[n - 1, n - 1]
    for k in range(n - 2, -1, -1):
        b[k] = (b[k] - np.dot(A[k, k + 1:n], b[k + 1:n])) / A[k, k]
    return b


def choleski_method(A, b):
    """
    Solve the linear system Ax = b using Choleski method.
    :param A: matrix
    :param b: vector
    :return: solution
    """
    n = len(A)
    for k in range(n):
        try:
            A[k, k] = np.sqrt(A[k, k] - np.dot(A[k, 0:k], A[k, 0:k]))
        except ValueError:
            raise ValueError("Matrix is not positive definite")
        for i in range(k + 1, n):
            A[i, k] = (A[i, k] - np.dot(A[i, 0:k], A[k, 0:k])) / A[k, k]
    for k in range(1, n):
        b[k] = b[k] - np.dot(A[k, 0:k], b[0:k])
    b[n - 1] = b[n - 1] / A[n - 1, n - 1]
    for k in range(n - 2, -1, -1):
        b[k] = (b[k] - np.dot(A[k, k + 1:n], b[k + 1:n])) / A[k, k]
    return b


def crout_algorithm(A, b):
    """
    Solve the linear system Ax = b using Crout algorithm.
    :param A: matrix
    :param b: vector
    :return: solution
    """
    pass
