def fizzbuzz(n):
    f = lambda i : \
            "FizzBuzz" if (i % 15) == 0 \
            else "Fizz" if (i % 3) == 0 \
            else "Buzz" if (i % 5) == 0 \
            else i

    return list(map(f, range(1, n + 1)))


def main():
    n = int(input("Input a number: "))
    ns = fizzbuzz(n)
    print(ns)


if __name__ == '__main__':
    main()
