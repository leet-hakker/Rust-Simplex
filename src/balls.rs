use crate::m::BigM;
use crate::fraction::Fraction;

#[no_mangle]
pub extern "C" fn balls() {
    let new_m = BigM::new(Fraction::new(10, 1), Fraction::new(0, 1));
    let m2 = BigM::new(Fraction::new(5, 1), Fraction::new(0, 1));
    println!("{}", new_m);
    println!("{}", m2);
    println!("{}", new_m * m2);
    println!("{}", new_m < BigM::new(Fraction::new(0, 1), Fraction::new(1, 1)));
}
