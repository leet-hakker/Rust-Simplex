use simplexsolver::simplex::{simplex, print_mat};
use simplexsolver::m::BigM;
use simplexsolver::matrix::Matrix;
use simplexsolver::fraction::Fraction;
use num_traits::Zero;

fn main() {
    let row_names = ["r".to_string(), "s".to_string(), "P".to_string()].to_vec();
    let column_names = ["x".to_string(), "y".to_string(), "r".to_string(), "s".to_string(), "Value".to_string()].to_vec();

    let data = vec![vec![BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(70, 1), Fraction::zero())],
    vec![BigM::new(Fraction::new(10, 1), Fraction::zero()), BigM::new(Fraction::new(3, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(60, 1), Fraction::zero())],
    vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]
    ];

    let mat = Matrix::new_with_data(3, 5, data);

    let (new_mat, row_names) = simplex(mat, row_names, &column_names);

    print_mat(&new_mat);

    println!("{:?}", row_names);
}
