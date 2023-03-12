use num_traits;
use num::integer::gcd;
use num::abs;
use std::ops;
use std::fmt;
use std::cmp;

#[derive(Clone, Copy, Eq, Debug)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Fraction {
        return Fraction {numerator, denominator}.simplify()
    }

    pub fn simplify(&mut self) -> Fraction {
        let divisor = gcd(abs(self.numerator), abs(self.denominator));
        self.numerator /= divisor;
        self.denominator /= divisor;
        *self
    }

    pub fn make_common(&mut self, frac2: &mut Fraction) {
        self.numerator *= frac2.denominator;
        self.denominator *= frac2.denominator;
        frac2.numerator *= self.denominator / frac2.denominator;
        frac2.denominator = self.denominator;
        
        assert_eq!(self.denominator, frac2.denominator);
    }

    pub fn abs(self) -> Fraction {
        return Fraction::new(abs(self.numerator), self.denominator);
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denominator == 1 || self.numerator == 0 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl ops::Add<Fraction> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Self::Output {
        let (mut temp1, mut temp2) = (self.clone(), rhs.clone());

        temp1.make_common(&mut temp2);
        assert_eq!(temp1.denominator, temp2.denominator);
        Fraction::new(temp1.numerator + temp2.numerator, temp1.denominator).simplify()
    }
}

impl ops::Sub<Fraction> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Self::Output {
        let (mut temp1, mut temp2) = (self.clone(), rhs.clone());

        temp1.make_common(&mut temp2);
        assert_eq!(temp1.denominator, temp2.denominator);
        Fraction::new(temp1.numerator - temp2.numerator, temp1.denominator).simplify()
    }
}

impl ops::Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Fraction::new(self.numerator * rhs.numerator, self.denominator * rhs.denominator).simplify()
    }
}

impl ops::Div<Fraction> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Self::Output {
        if self.denominator * rhs.numerator < 0 {
            return Fraction::new(-self.numerator * rhs.denominator, self.denominator * -rhs.numerator).simplify()
        } else {
            return Fraction::new(self.numerator * rhs.denominator, self.denominator * rhs.numerator).simplify()
        }
    }
}

impl cmp::PartialEq<Fraction> for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        let (mut temp1, mut temp2) = (self.clone(), other.clone());
        temp1.make_common(&mut temp2);
        temp1.numerator == temp2.numerator
    }

    fn ne(&self, other: &Fraction) -> bool {
        let (mut temp1, mut temp2) = (self.clone(), other.clone());
        temp1.make_common(&mut temp2);
        temp1.numerator != temp2.numerator
    } 
}

impl cmp::PartialOrd<Fraction> for Fraction {
    fn partial_cmp(&self, other: &Fraction) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Fraction) -> bool {
        let (mut temp1, mut temp2) = (self.clone(), other.clone());
        temp1.make_common(&mut temp2);
        temp1.numerator < temp2.numerator
    }

    fn le(&self, other: &Fraction) -> bool {
        let (mut temp1, mut temp2) = (self.clone(), other.clone());
        temp1.make_common(&mut temp2);
        temp1.numerator <= temp2.numerator 
    }

    fn gt(&self, other: &Fraction) -> bool {
        let (mut temp1, mut temp2) = (self.clone(), other.clone());
        temp1.make_common(&mut temp2);
        temp1.numerator > temp2.numerator 
    }

    fn ge(&self, other: &Fraction) -> bool {
        let (mut temp1, mut temp2) = (self.clone(), other.clone());
        temp1.make_common(&mut temp2);
        temp1.numerator >= temp2.numerator 
    }
}

impl cmp::Ord for Fraction {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let (mut temp1, mut temp2) = (self.clone(), other.clone());
        temp1.make_common(&mut temp2);
        self.numerator.cmp(&other.numerator)
    }
}

impl num_traits::Zero for Fraction {
    fn zero() -> Self {
        Fraction::new(0, 1)
    }

    fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

trait S: Sized {}

impl S for Fraction {}