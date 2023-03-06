import unittest
from fizzbuzz import do_single_fb, check_number

class TestFB(unittest.TestCase):

    # Valid cases
    def test_fb_valid1(self):
        self.assertEqual(do_single_fb(1), '1')

    def test_fb_valid2(self):
        self.assertEqual(do_single_fb(2), '2')

    def test_fb_valid3(self):
        self.assertEqual(do_single_fb(3), 'Fizz')

    def test_fb_valid4(self):
        self.assertEqual(do_single_fb(4), '4')

    def test_fb_valid5(self):
        self.assertEqual(do_single_fb(5), 'Buzz')

    def test_fb_valid6(self):
        self.assertEqual(do_single_fb(15), 'FizzBuzz')

class TestCheckNum(unittest.TestCase):

    def test_cn_valid1(self):
        self.assertEqual(check_number(1), 1)

    def test_cn_valid2(self):
        self.assertEqual(check_number('1'), 1)

    def test_cn_invalid1(self):
        self.assertRaises(ValueError, check_number, 0)

    def test_cn_invalid2(self):
        self.assertRaises(ValueError, check_number, '0')

    def test_cn_invalid3(self):
        self.assertRaises(ValueError, check_number, -1)

    def test_cn_invalid4(self):
        self.assertRaises(ValueError, check_number, '-1')

    def test_cn_invalid5(self):
        self.assertRaises(ValueError, check_number, 'a')

    def test_cn_invalid6(self):
        self.assertRaises(ValueError, check_number, ' ')

    def test_cn_invalid7(self):
        self.assertRaises(ValueError, check_number, '')

    def test_cn_invalid8(self):
        self.assertRaises(ValueError, check_number, 1.2)

    def test_cn_invalid9(self):
        self.assertRaises(ValueError, check_number, '1.2')

    def test_cn_invalid10(self):
        self.assertRaises(ValueError, check_number, -1.2)

    def test_cn_invalid11(self):
        self.assertRaises(ValueError, check_number, '-1.2')        

if __name__ == '__main__':
    unittest.main()