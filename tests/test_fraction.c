#include <stdlib.h>
#include <check.h>
#include "../include/fraction.h"

START_TEST(test_gcd)
{
    ck_assert_int_eq(gcd(12, 4), 4);
    ck_assert_int_eq(gcd(-12, 4), 4);
    ck_assert_int_eq(gcd(0, 5), 5);
    ck_assert_int_eq(gcd(13, 5), 1);
    ck_assert_int_eq(gcd(15, 10), 5);
}
END_TEST

START_TEST(test_simplify_fraction)
{
    Fraction frac1 = {2, 4};
    Fraction frac2 = {-6, 9};
    Fraction frac3 = {0, 5};
    Fraction frac4 = {10, 0};
    simplify_fraction(&frac1);
    simplify_fraction(&frac2);
    simplify_fraction(&frac3);
    simplify_fraction(&frac4);
    ck_assert_int_eq(frac1.numerator, 1);
    ck_assert_int_eq(frac1.denominator, 2);
    ck_assert_int_eq(frac2.numerator, -2);
    ck_assert_int_eq(frac2.denominator, 3);
    ck_assert_int_eq(frac3.numerator, 0);
    ck_assert_int_eq(frac3.denominator, 1);
    ck_assert_int_eq(frac4.numerator, 1);
    ck_assert_int_eq(frac4.denominator, 0);
}
END_TEST

START_TEST(test_new_fraction)
{
    Fraction frac1 = new_fraction(4, 8);
    Fraction frac2 = new_fraction(-6, 9);
    Fraction frac3 = new_fraction(0, 5);
    ck_assert_int_eq(frac1.numerator, 1);
    ck_assert_int_eq(frac1.denominator, 2);
    ck_assert_int_eq(frac2.numerator, -2);
    ck_assert_int_eq(frac2.denominator, 3);
    ck_assert_int_eq(frac3.numerator, 0);
    ck_assert_int_eq(frac3.denominator, 1);
}
END_TEST

START_TEST(test_fraction_str)
{
    Fraction frac1 = {2, 1};
    Fraction frac2 = {-6, 9};
    Fraction frac3 = {0, 5};
    Fraction frac4 = {10, 3};
    char *str = calloc(20, sizeof(char));
    ck_assert_str_eq(fraction_str(frac1, str), "2");
    ck_assert_str_eq(fraction_str(frac2, str), "-6/9");
    ck_assert_str_eq(fraction_str(frac3, str), "0");
    ck_assert_str_eq(fraction_str(frac4, str), "10/3");
    free(str);
}
END_TEST

START_TEST(test_fraction_zero)
{
    Fraction frac = fraction_zero();
    ck_assert_int_eq(frac.numerator, 0);
    ck_assert_int_eq(frac.denominator, 1);
}
END_TEST

START_TEST(test_fraction_is_zero)
{
    Fraction frac1 = {0, 1};
    Fraction frac2 = {2, 1};
    ck_assert(fraction_is_zero(frac1));
    ck_assert(!fraction_is_zero(frac2));
}
END_TEST

START_TEST(test_make_common_denominator) {
    Fraction frac1 = {1, 2};
    Fraction frac2 = {3, 4};
    make_common_denominator(&frac1, &frac2);
    ck_assert_int_eq(frac1.denominator, 8);
    ck_assert_int_eq(frac2.denominator, 8);
    ck_assert_int_eq(frac1.numerator, 4);
    ck_assert_int_eq(frac2.numerator, 6);

    frac1 = (Fraction) {-1, 2};
    frac2 = (Fraction) {3, -4};
    make_common_denominator(&frac1, &frac2);
    ck_assert_int_eq(frac1.denominator, -8);
    ck_assert_int_eq(frac2.denominator, -8);
    ck_assert_int_eq(frac1.numerator, 4);
    ck_assert_int_eq(frac2.numerator, 6);
}
END_TEST

START_TEST(test_abs_fraction) {
    Fraction frac1 = {1, 2};
    Fraction frac2 = {-1, 2};
    Fraction frac3 = {1, -2};
    Fraction frac4 = {-1, -2};

    Fraction abs_frac1 = abs_fraction(frac1);
    Fraction abs_frac2 = abs_fraction(frac2);
    Fraction abs_frac3 = abs_fraction(frac3);
    Fraction abs_frac4 = abs_fraction(frac4);

    ck_assert_int_eq(abs_frac1.numerator, 1);
    ck_assert_int_eq(abs_frac1.denominator, 2);
    ck_assert_int_eq(abs_frac2.numerator, 1);
    ck_assert_int_eq(abs_frac2.denominator, 2);
    ck_assert_int_eq(abs_frac3.numerator, 1);
    ck_assert_int_eq(abs_frac3.denominator, 2);
    ck_assert_int_eq(abs_frac4.numerator, 1);
    ck_assert_int_eq(abs_frac4.denominator, 2);
}
END_TEST

START_TEST(test_fraction_eq) {
    Fraction frac1 = {1, 2};
    Fraction frac2 = {3, 4};
    Fraction frac3 = {6, 8};
    Fraction frac4 = {-1, -2};

    ck_assert(!fraction_eq(frac1, frac3));
    ck_assert(fraction_eq(frac2, frac3));
    ck_assert(!fraction_eq(frac1, frac2));
    ck_assert(fraction_eq(frac1, frac4));
}
END_TEST

START_TEST(test_fraction_ne) {
    Fraction frac1 = {1, 2};
    Fraction frac2 = {3, 4};
    Fraction frac3 = {1, 2};

    ck_assert(fraction_ne(frac1, frac2));
    ck_assert(!fraction_ne(frac1, frac3));
}
END_TEST

START_TEST(test_fraction_lt) {
    Fraction frac1 = {1, 2};
    Fraction frac2 = {3, 4};
    Fraction frac3 = {5, 6};

    ck_assert(fraction_lt(frac1, frac2));
    ck_assert(!fraction_lt(frac2, frac1));
    ck_assert(fraction_lt(frac1, frac3));
    ck_assert(fraction_lt(frac2, frac3));
}
END_TEST

START_TEST(test_fraction_gt) {
    Fraction a = new_fraction(1, 2);
    Fraction b = new_fraction(1, 3);
    Fraction c = new_fraction(-1, 2);
    Fraction d = new_fraction(2, 3);

    ck_assert(fraction_gt(a, b));
    ck_assert(!fraction_gt(b, a));
    ck_assert(!fraction_gt(c, a));
    ck_assert(fraction_gt(a, c));
    ck_assert(fraction_gt(d, a));
    ck_assert(!fraction_gt(a, d));
}
END_TEST

START_TEST(test_fraction_gte) {
    Fraction a = new_fraction(1, 2);
    Fraction b = new_fraction(1, 3);
    Fraction c = new_fraction(-1, 2);
    Fraction d = new_fraction(2, 3);

    ck_assert(fraction_gte(a, a));
    ck_assert(fraction_gte(a, b));
    ck_assert(!fraction_gte(b, a));
    ck_assert(!fraction_gte(c, a));
    ck_assert(fraction_gte(a, c));
    ck_assert(fraction_gte(d, a));
    ck_assert(!fraction_gte(a, d));
}
END_TEST

START_TEST(test_fraction_lte) {
    Fraction a = new_fraction(1, 2);
    Fraction b = new_fraction(1, 3);
    Fraction c = new_fraction(-1, 2);
    Fraction d = new_fraction(2, 3);

    ck_assert(fraction_lte(a, a));
    ck_assert(!fraction_lte(a, b));
    ck_assert(fraction_lte(b, a));
    ck_assert(fraction_lte(c, a));
    ck_assert(!fraction_lte(a, c));
    ck_assert(!fraction_lte(d, a));
    ck_assert(fraction_lte(a, d));

}

START_TEST(test_fraction_add)
{
    Fraction f1 = new_fraction(1, 2);
    Fraction f2 = new_fraction(1, 3);
    Fraction result = fraction_add(f1, f2);
    Fraction expected = new_fraction(5, 6);
    ck_assert(fraction_eq(result, expected));

    f1 = new_fraction(-1, 2);
    f2 = new_fraction(1, 3);
    result = fraction_add(f1, f2);
    expected = new_fraction(-1, 6);
    ck_assert(fraction_eq(result, expected));
}
END_TEST

START_TEST(test_fraction_subtract)
{
    Fraction f1 = new_fraction(1, 2);
    Fraction f2 = new_fraction(1, 3);
    Fraction result = fraction_subtract(f1, f2);
    Fraction expected = new_fraction(1, 6);
    ck_assert(fraction_eq(result, expected));

    f1 = new_fraction(-1, 2);
    f2 = new_fraction(1, 3);
    result = fraction_subtract(f1, f2);
    expected = new_fraction(-5, 6);
    ck_assert(fraction_eq(result, expected));
}
END_TEST

START_TEST(test_fraction_multiply)
{
    Fraction f1 = new_fraction(2, 3);
    Fraction f2 = new_fraction(4, 5);
    Fraction result = fraction_multiply(f1, f2);
    Fraction expected = new_fraction(8, 15);
    ck_assert(fraction_eq(result, expected));
    
    f1 = new_fraction(-2, 3);
    f2 = new_fraction(4, -5);
    result = fraction_multiply(f1, f2);
    expected = new_fraction(8, 15);
    ck_assert(fraction_eq(result, expected));
    
    f1 = new_fraction(2, 3);
    f2 = new_fraction(0, 5);
    result = fraction_multiply(f1, f2);
    expected = fraction_zero();
    ck_assert(fraction_eq(result, expected));
}
END_TEST

START_TEST(test_fraction_divide)
{
    Fraction f1 = new_fraction(2, 3);
    Fraction f2 = new_fraction(4, 5);
    Fraction result = fraction_divide(f1, f2);
    Fraction expected = new_fraction(5, 6);
    ck_assert(fraction_eq(result, expected));
    
    f1 = new_fraction(-2, 3);
    f2 = new_fraction(4, -5);
    result = fraction_divide(f1, f2);
    expected = new_fraction(5, 6);
    ck_assert(fraction_eq(result, expected));
    
    f1 = new_fraction(0, 5);
    f2 = new_fraction(2, 3);
    result = fraction_divide(f1, f2);
    expected = fraction_zero();
    ck_assert(fraction_eq(result, expected));
}
END_TEST

Suite *fraction_suite() {
    Suite *s;
    TCase *tc_core;

    s = suite_create("fraction");

    tc_core = tcase_create("Core");

    tcase_add_test(tc_core, test_new_fraction);
    tcase_add_test(tc_core, test_abs_fraction);
    tcase_add_test(tc_core, test_fraction_add);
    tcase_add_test(tc_core, test_fraction_divide);
    tcase_add_test(tc_core, test_fraction_eq);
    tcase_add_test(tc_core, test_fraction_gt);
    tcase_add_test(tc_core, test_fraction_gte);
    tcase_add_test(tc_core, test_fraction_is_zero);
    tcase_add_test(tc_core, test_fraction_lt);
    tcase_add_test(tc_core, test_fraction_lte);
    tcase_add_test(tc_core, test_fraction_multiply);
    tcase_add_test(tc_core, test_fraction_ne);
    tcase_add_test(tc_core, test_fraction_str);
    tcase_add_test(tc_core, test_fraction_subtract);
    tcase_add_test(tc_core, test_fraction_zero);
    tcase_add_test(tc_core, test_gcd);
    tcase_add_test(tc_core, test_make_common_denominator);
    tcase_add_test(tc_core, test_simplify_fraction);
    suite_add_tcase(s, tc_core);

    return s;
}

int main() {
    int number_failed;
    Suite *s;
    SRunner *sr;

    // Create test suite
    s = fraction_suite();
    // Create test runner
    sr = srunner_create(s);

    // Run the tests
    srunner_run_all(sr, CK_NORMAL);

    // Get the number of failed tests
    number_failed = srunner_ntests_failed(sr);

    // Free memory
    srunner_free(sr);

    // Return success or failure
    return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}