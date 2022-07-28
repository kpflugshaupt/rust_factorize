import math

import rust_factorize as fact


def prime_factors(n: int) -> list[int]:
    """
    Decompose a number into its constituent prime factors.

    :param n: Number to decompose
    :return: a list of prime factors
    """
    i: int = 3
    factors: list[int] = []
    max_factor = int(math.sqrt(n)) + 1
    while not n % 2:
        factors.append(2)
        n //= 2
    while i <= max_factor and n > 1:
        while not n % i:
            n //= i
            factors.append(i)
        i += 2
    if n > 1:
        factors.append(n)
    return factors


factors1 = fact.to_primes(1000)
factors2 = prime_factors(1000)

assert factors1 == factors2
