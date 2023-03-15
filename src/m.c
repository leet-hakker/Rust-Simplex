#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "../include/fraction.h"

typedef struct BigM {
    Fraction num_val;
    Fraction m_val;
} BigM;

BigM new_BigM(Fraction num_val, Fraction m_val) {
    BigM bm = {num_val, m_val};
    return bm;
}

BigM BigM_zero() {
    return new_BigM(fraction_zero(), fraction_zero());
}

BigM BigM_one() {
    return new_BigM(new_fraction(1, 1), fraction_zero());
}

char *BigM_str(BigM bm) {
    char *temp = calloc(25, 1);
    char *str;
    if (!fraction_is_zero(bm.num_val)) {
        str = fraction_str(bm.num_val);
        strcat(temp, str);
        free(str);
        if (!fraction_is_zero(bm.m_val)) {
            if (fraction_lt(bm.m_val, fraction_zero())) {
                strcat(temp, " - ");
                str = fraction_str(abs_fraction(bm.m_val));
                strcat(temp, str);
                free(str);
                strcat(temp, "M");
            } else {
                strcat(temp, " + ");
                str = fraction_str(bm.m_val);
                strcat(temp, str);
                free(str);
                strcat(temp, "M");
            }
        }
    } else if (!fraction_is_zero(bm.m_val)) {
        str = fraction_str(bm.m_val);
        strcat(temp, str);
        free(str);
        strcat(temp, "M");
    } else {
        strcat(temp, "0");
    }

    return temp;
}

BigM add_BigM(BigM bm1, BigM bm2) {
    return new_BigM(fraction_add(bm1.num_val, bm2.num_val), fraction_add(bm1.m_val, bm2.m_val));
}

BigM sub_BigM(BigM bm1, BigM bm2) {
    return new_BigM(fraction_subtract(bm1.num_val, bm2.num_val), fraction_subtract(bm1.m_val, bm2.m_val));
}

BigM mul_BigM(BigM bm1, BigM bm2) {
    if (fraction_is_zero(bm2.num_val) && !fraction_is_zero(bm2.m_val) && fraction_is_zero(bm1.m_val)) {
        return new_BigM(fraction_zero(), fraction_multiply(bm1.num_val, bm2.m_val));
    }
    if (!fraction_is_zero(bm2.m_val)) {
        printf("Attempt to multiply ");
        printf("%s", BigM_str(bm1));
        printf(" by ");
        printf("%s", BigM_str(bm2));
        printf(", which has non-zero M value");
        exit(EXIT_FAILURE);
    }
    return new_BigM(fraction_multiply(bm1.num_val, bm2.num_val), fraction_multiply(bm1.m_val, bm2.num_val));
}

BigM div_BigM(BigM bm1, BigM bm2) {
    if (fraction_is_zero(bm1.num_val) && fraction_is_zero(bm2.num_val)) {
        if (!fraction_is_zero(bm1.m_val) && !fraction_is_zero(bm2.m_val)) {
            return new_BigM(fraction_divide(bm1.m_val, bm2.m_val), fraction_zero());
        }
    }
    if (!fraction_is_zero(bm2.m_val)) {
        printf("Attempt to divide by ");
        printf("%s", BigM_str(bm2));
        printf(", which has non-zero M value");
        exit(EXIT_FAILURE);
    }
    return new_BigM(fraction_divide(bm1.num_val, bm2.num_val), fraction_divide(bm1.m_val, bm2.num_val));
}

bool BigM_eq(BigM bm1, BigM bm2) {
    return fraction_eq(bm1.m_val, bm2.m_val) && fraction_eq(bm1.num_val, bm2.num_val);
}

bool BigM_ne(BigM bm1, BigM bm2) {
    return fraction_ne(bm1.m_val, bm2.m_val) || fraction_ne(bm1.num_val, bm2.num_val);
}

bool BigM_lt(BigM* a, BigM* b) {
    if (fraction_gt(b->m_val, a->m_val)) {
        return true;
    } else if (fraction_eq(b->m_val, a->m_val)) {
        return fraction_gt(b->num_val, a->num_val);
    } else {
        return false;
    }
}

bool BigM_le(BigM* a, BigM* b) {
    if (fraction_eq(a->m_val, b->m_val)) {
        return fraction_lte(a->num_val, b->num_val);
    } else if (fraction_lt(a->m_val, b->m_val)) {
        return true;
    } else {
        return false;
    }
}

bool BigM_gt(BigM* a, BigM* b) {
    if (fraction_lt(b->m_val, a->m_val)) {
        return true;
    } else if (fraction_eq(b->m_val, a->m_val)) {
        return fraction_lt(b->num_val, a->num_val);
    } else {
        return false;
    }
}

bool BigM_ge(BigM* a, BigM* b) {
    if (fraction_eq(a->m_val, b->m_val)) {
        return fraction_gte(a->num_val, b->num_val);
    } else if (fraction_gt(a->m_val, b->m_val)) {
        return true;
    } else {
        return false;
    }
}