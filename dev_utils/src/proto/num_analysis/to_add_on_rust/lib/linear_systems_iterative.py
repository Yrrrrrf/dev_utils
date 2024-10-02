import numpy as np


'''
Linear Systems are a set of equations with the same variables.

Linear Systems Methods:
    - Jacobi Method (7.1)
    - Gauss-Seidel Method (7.2)

# todo: pending implementaions:
    - Successive Over-Relaxation (SOR) (7.3)
    - Iterative Refinement (7.4)
    - Conjugate Gradient Method (7.5)
'''


def jacobi_method(A, b, tol=1.0e-9, maxiter=100):
    """Jacobi method for solving linear systems Ax = b.
    :param A: matrix
    :param b: vector
    :param x0: initial guess
    :param tol: tolerance
    :param maxiter: maximum number of iterations
    :return: solution
    """
    n = len(A)
    x = np.zeros(n)
    for _ in range(maxiter):
        xold = x.copy()
        for i in range(n):
            s1 = np.dot(A[i, 0:i], x[0:i])
            s2 = np.dot(A[i, i + 1:n], xold[i + 1:n])
            x[i] = (b[i] - s1 - s2) / A[i, i]
        if np.linalg.norm(x - xold) < tol: return x  # type: ignore
    print("Jacobi method did not converge")
    return x


def gauss_seidel_method(A, b, x0, tol=1.0e-9, maxiter=100):
    """
    Solve the linear system Ax = b using Gauss-Seidel method.
    :param A: matrix
    :param b: vector
    :param x0: initial guess
    :param tol: tolerance
    :param maxiter: maximum number of iterations
    :return: solution
    """
    n = len(A)
    x = x0.copy()
    for _ in range(maxiter):
        xold = x.copy()
        for i in range(n):
            s1 = np.dot(A[i, 0:i], x[0:i])
            s2 = np.dot(A[i, i + 1:n], x[i + 1:n])
            x[i] = (b[i] - s1 - s2) / A[i, i]
        if np.linalg.norm(x - xold) < tol: return x
    print("Gauss-Seidel method did not converge")
    return x


if __name__ == "__main__":
    # 2x + y = 1
    # x + 2y = 2
    A = np.array([[2, 1], [1, 2]])
    b = np.array([1, 2])
    x0 = np.array([0, 0])

    print(f'Jacobi Method: {jacobi_method(A, b)}')
    print(f'Gauss-Seidel Method: {gauss_seidel_method(A, b, x0)}')
