import unittest

from fib import fib


class T(unittest.TestCase):
    def test_fib(self) -> None:
        self.assertEqual(fib(20), 6765)


if __name__ == "__main__":
    unittest.main()
