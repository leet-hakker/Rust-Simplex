use crate::fraction::Fraction;
use std::fmt;
use std::ops;
use std::cmp;

#[derive(Clone, Copy, Eq)]
pub struct BigM {
    num_val: Fraction,
    m_val: Fraction,
}

impl BigM {
    #[no_mangle]
    pub fn new(num_val: Fraction, m_val: Fraction) -> BigM {
        BigM { num_val, m_val }
    }
}

impl fmt::Display for BigM {
    #[no_mangle]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num_val.numerator != 0 {
            write!(f, "{}", self.num_val).expect("Failed to write num_val to buffer");
            if self.m_val.numerator != 0 {
                if self.m_val < Fraction::new(0, 1) {
                    return write!(f, " - {}M", self.m_val.abs());
                } else {
                    return write!(f, " + {}M", self.m_val);
                }
            }
        } else if self.m_val.numerator != 0 {
            write!(f, "{}M", self.m_val).expect("Failed to write m_val to buffer");
        } else {
            return write!(f, "0");
        }
        write!(f, "")
    }
}

impl ops::Add<BigM> for BigM {
    type Output = BigM;
    #[no_mangle]
    fn add(self, rhs: BigM) -> Self::Output {
        BigM::new(self.num_val + rhs.num_val, self.m_val + rhs.m_val)
    }
}

impl ops::Sub<BigM> for BigM {
    type Output = BigM;
    #[no_mangle]
    fn sub(self, rhs: BigM) -> Self::Output {
        BigM::new(self.num_val - rhs.num_val, self.m_val - rhs.num_val)
    }
}

impl ops::Mul<BigM> for BigM {
    type Output = BigM;
    #[no_mangle]
    fn mul(self, rhs: BigM) -> Self::Output {
        if rhs.num_val.numerator == 0 && rhs.m_val.numerator != 0 && self.m_val.numerator == 0 {
            return BigM::new(Fraction::new(0, 1), self.num_val * rhs.m_val);
        }
        if rhs.m_val.numerator != 0 {
            panic!("Attempt to multiply {} by {}, which has non-zero M value", self, rhs);
        }
        return BigM::new(self.num_val * rhs.num_val, self.m_val * rhs.num_val);
    }
}

impl ops::Div<BigM> for BigM {
    type Output = BigM;
    #[no_mangle]
    fn div(self, rhs: BigM) -> Self::Output {
        if self.num_val.numerator == 0 && rhs.num_val.numerator == 0 {
            if self.m_val.numerator != 0 && rhs.m_val.numerator != 0 {
                return BigM::new(self.m_val / rhs.m_val, Fraction::new(0, 1));
            }
        }
        if rhs.m_val.numerator != 0 {
            panic!("Attempt to divide by {}, which has non-zero M value", rhs);
        }
        return BigM::new(self.num_val / rhs.num_val, self.m_val / rhs.num_val);
    }
}

impl cmp::PartialEq<BigM> for BigM {
    #[no_mangle]
    fn eq(&self, other: &BigM) -> bool {
        return self.m_val == other.m_val && self.num_val == other.num_val;
    }

    #[no_mangle]
    fn ne(&self, other: &BigM) -> bool {
        return self.m_val != other.m_val || self.num_val != other.num_val;
    } 
}

impl cmp::PartialOrd<BigM> for BigM {
    #[no_mangle]
    fn partial_cmp(&self, other: &BigM) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[no_mangle]
    fn lt(&self, other: &BigM) -> bool {
        if other.m_val > self.m_val {
            return true;
        } else if other.m_val == self.m_val {
            return other.num_val > self.num_val;
        }
        return false;
    }

    #[no_mangle]
    fn le(&self, other: &BigM) -> bool {
        if self.m_val <= other.m_val {
            return self.num_val < other.num_val;
        } else {
            return false;
        }
    }

    #[no_mangle]
    fn gt(&self, other: &BigM) -> bool {
        if other.m_val < self.m_val {
            return true;
        } else if other.m_val == self.m_val {
            return other.num_val < self.num_val;
        }
        return false;
    }

    #[no_mangle]
    fn ge(&self, other: &BigM) -> bool {
        if self.m_val >= other.m_val {
            return self.num_val > other.num_val;
        } else {
            return false;
        }
    }
}

impl cmp::Ord for BigM {
    #[no_mangle]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.m_val != other.m_val {
            self.m_val.cmp(&other.m_val)
        } else {
            self.num_val.cmp(&other.num_val)
        }
    }
}