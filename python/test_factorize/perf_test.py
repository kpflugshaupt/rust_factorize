import math

import rust_factorize as fact


def prime_factors(n: int) -> list[int]:
    i: int = 3
    factors: list[int] = []
    max_factor = int(math.sqrt(n)) + 1
    while not n % 2:
        factors.append(2)
        n //= 2
    while i <= max_factor:
        if n % i:
            i += 2
        else:
            n //= i
            factors.append(i)
    if n > 1:
        factors.append(n)
    return factors


factors1 = fact.to_primes(1000)
factors2 = prime_factors(1000)

assert factors1 == factors2
