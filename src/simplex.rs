use crate::m::BigM;
use crate::fraction::Fraction;
use crate::matrix::Matrix;
use num_traits::Zero;

// pub fn data_into_mat(data: Vec<BigM>, size: (usize, usize)) -> Conventional<BigM> { 
//     let mut mat = Conventional::new(size);
//     for i in 0..mat.rows {
//         for j in 0..mat.columns {
//             mat.values[j*mat.rows + i] = data[j*3 + i];
//         }
//     }
//     mat
// }

pub fn print_mat(matrix: &Matrix) {
    for i in 0..matrix.rows {
        for j in 0..matrix.columns {
            print!("{}  ", matrix.get(i, j));
        }
        println!("");
    }
    println!("");
}

pub fn isoptimal(matrix: &Matrix) -> bool {
    let mut minimum: BigM = matrix.get(matrix.rows-1, 0);
    let mut value: BigM;

    for i in 0..matrix.columns {
        value = matrix.get(matrix.rows-1, i);
        if value < minimum { 
            minimum = value;
        }
    }

    return minimum >= BigM::zero();
}

pub fn pivot_column(matrix: &Matrix) -> usize {
    let mut minimum: BigM = matrix.get(matrix.rows-1, 0);
    let mut index: usize = 0;
    let mut value: BigM;

    for i in 0..matrix.columns-1 {
        value = matrix.get(matrix.rows-1, i);
        if value < minimum { 
            minimum = value;
            index = i;
        }
    }

    return index;
}

pub fn pivot_row(matrix: &Matrix, pivt_col: &usize) -> usize {
    let mut theta: BigM = BigM::new(Fraction::zero(), Fraction::new(1, 1));
    let mut minimum: BigM = theta;
    let mut index: usize = 0;
    let mut value: BigM;

    for row in 0..matrix.rows-1 {
        value = matrix.get(row, *pivt_col);
        
        // If value is 0, division by it will cause an error. We want to avoid this case.
        // We know that as values approach 0, the result of dividing by this value increases.
        // Thus for this case we assume that dividing by zero will result in an infinitely large
        // value. We want to select the smallest theta value, so we can immediately ignore
        // any values of 0.
        if value.is_zero() {continue}
        theta = matrix.get(row, matrix.columns-1) / value;
        if theta >= BigM::zero() && theta < minimum {
            minimum = theta;
            index = row;
        }
    }

    return index;
}

pub fn simplex(mut matrix: Matrix, mut row_names: Vec<String>, column_names: &Vec<String>) -> (Matrix, Vec<String>) {
    let mut column: usize;
    let mut row: usize;
    
    while !isoptimal(&matrix) {
        column = pivot_column(&matrix);
        row = pivot_row(&matrix, &column);
        let pivot_value = matrix.get(row, column);

        print_mat(&matrix);

        for i in 0..matrix.columns {
            let current_column_value = matrix.get(row, i);
            matrix.set(row, i, current_column_value / pivot_value);
        }

        assert_eq!(matrix.get(row, column), BigM::one());

        print_mat(&matrix);

        row_names[row] = (*column_names[column]).to_string();

        for i in 0..matrix.rows {
            if i == row {continue}
            let current_row_pivot_column_value = matrix.get(i, column);

            for j in 0..matrix.columns {
                let pivot_row_value = matrix.get(row, j);
                let current_cell_value = matrix.get(i, j);
                matrix.set(i, j, current_cell_value - (current_row_pivot_column_value * pivot_row_value));
            }
            assert_eq!(matrix.get(i, column), BigM::zero());
        }
        print_mat(&matrix);
    }

    return (matrix, row_names);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isoptimal() {
        let data1: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(70, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(10, 1), Fraction::zero()), BigM::new(Fraction::new(3, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(60, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]];
    
        let solution1: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(2, 11), Fraction::zero()), BigM::new(Fraction::new(-1, 11), Fraction::zero()), BigM::new(Fraction::new(80, 11), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-3, 11), Fraction::zero()), BigM::new(Fraction::new(7, 55), Fraction::zero()), BigM::new(Fraction::new(42, 11), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 5), Fraction::zero()), BigM::new(Fraction::new(1, 5), Fraction::zero()), BigM::new(Fraction::new(26, 1), Fraction::zero())]
        ];
    
        let data2: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(8, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(6, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-4, 1), Fraction::zero()), BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]
                                        ];

        let solution2: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(16, 7), Fraction::zero()), BigM::new(Fraction::new(2, 7), Fraction::zero()), BigM::new(Fraction::new(3, 7), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(32, 7), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(6, 7), Fraction::zero()), BigM::new(Fraction::new(-1, 7), Fraction::zero()), BigM::new(Fraction::new(2, 7), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(37, 7), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-13, 7), Fraction::zero()), BigM::new(Fraction::new(1, 7), Fraction::zero()), BigM::new(Fraction::new(-2, 7), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(30, 1), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(107, 7), Fraction::zero()), BigM::new(Fraction::new(2, 7), Fraction::zero()), BigM::new(Fraction::new(17, 7), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(144, 7), Fraction::zero())]
        ];
    
        let data3: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(20, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-1, 1), Fraction::new(-1, 1)), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(1, 1)), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(-4, 1))]
        ];
    
        let solution3: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::new(12, 1), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(3, 1), Fraction::zero()), BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(15, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::new(1, 1)), BigM::new(Fraction::new(16, 1), Fraction::zero())]
        ];
    
        let mat1: Matrix = Matrix::new_with_data(3, 5, data1);
        let ans1: Matrix = Matrix::new_with_data(3, 5, solution1);
        let mat2: Matrix = Matrix::new_with_data(4, 7, data2);
        let ans2: Matrix = Matrix::new_with_data(4, 7, solution2);
        let mat3: Matrix = Matrix::new_with_data(4, 8, data3);
        let ans3: Matrix = Matrix::new_with_data(4, 8, solution3);
        // let mat4: Conventional<BigM> = data_into_mat(data4, ());
        // let ans4: Conventional<BigM> = data_into_mat(solution4, ());
    

        assert!(!isoptimal(&mat1));
        assert!(isoptimal(&ans1));
        assert!(!isoptimal(&mat2));
        assert!(isoptimal(&ans2));
        assert!(!isoptimal(&mat3));
        assert!(isoptimal(&ans3));
    }

    #[test]
    fn test_pivot_column() {
        let data1: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(70, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(10, 1), Fraction::zero()), BigM::new(Fraction::new(3, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(60, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]];

        const PVT_COL1: usize = 0;
    
        let data2: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(8, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(6, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-4, 1), Fraction::zero()), BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]
                                        ];
    
        const PVT_COL2: usize = 1;
    
        let data3: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(20, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-1, 1), Fraction::new(-1, 1)), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(1, 1)), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(-4, 1))]
        ];

        const PVT_COL3: usize = 0;
    
        let mat1: Matrix = Matrix::new_with_data(3, 5, data1);
        let mat2: Matrix = Matrix::new_with_data(4, 7, data2);
        let mat3: Matrix = Matrix::new_with_data(4, 8, data3);
        // let mat4: Conventional<BigM> = data_into_mat(data4, ());
        // let ans4: Conventional<BigM> = data_into_mat(solution4, ());

        assert_eq!(pivot_column(&mat1), PVT_COL1);
        assert_eq!(pivot_column(&mat2), PVT_COL2);
        assert_eq!(pivot_column(&mat3), PVT_COL3);
        // assert_eq!(pivot_column(&mat4), PVT_COL4);
    }

    #[test]
    fn test_pivot_row() {
        let data1: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(70, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(10, 1), Fraction::zero()), BigM::new(Fraction::new(3, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(60, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]];

        const PVT_COL1: usize = 0;
        const PVT_ROW1: usize = 1;
    
        let data2: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(8, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(6, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-4, 1), Fraction::zero()), BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]
                                        ];
    
        const PVT_COL2: usize = 1;
        const PVT_ROW2: usize = 1;
    
        let data3: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(20, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-1, 1), Fraction::new(-1, 1)), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(1, 1)), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(-4, 1))]
        ];

        const PVT_COL3: usize = 0;
        const PVT_ROW3: usize = 2;
    
        let mat1: Matrix = Matrix::new_with_data(3, 5, data1);
        let mat2: Matrix = Matrix::new_with_data(4, 7, data2);
        let mat3: Matrix = Matrix::new_with_data(4, 8, data3);
        // let mat4: Conventional<BigM> = data_into_mat(data4, ());
        // let ans4: Conventional<BigM> = data_into_mat(solution4, ());

        assert_eq!(pivot_row(&mat1, &PVT_COL1), PVT_ROW1);
        assert_eq!(pivot_row(&mat2, &PVT_COL2), PVT_ROW2);
        assert_eq!(pivot_row(&mat3, &PVT_COL3), PVT_ROW3);
        // assert_eq!(pivot_row(&mat4, &PVT_COL4), PVT_ROW4);
    }

    #[test]
    fn test_simplex() {
        let data1: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(70, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(10, 1), Fraction::zero()), BigM::new(Fraction::new(3, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(60, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]];
    
        let columns1: Vec<String> = vec!["x".to_string(), "y".to_string(), "r".to_string(), "s".to_string(), "Value".to_string()];
        let rows1: Vec<String> = vec!["r".to_string(), "s".to_string(), "P".to_string()];
        let row_ans1: Vec<String> = vec!["y".to_string(), "x".to_string(), "P".to_string()];

        let solution1: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(2, 11), Fraction::zero()), BigM::new(Fraction::new(-1, 11), Fraction::zero()), BigM::new(Fraction::new(80, 11), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-3, 55), Fraction::zero()), BigM::new(Fraction::new(7, 55), Fraction::zero()), BigM::new(Fraction::new(42, 11), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 5), Fraction::zero()), BigM::new(Fraction::new(1, 5), Fraction::zero()), BigM::new(Fraction::new(26, 1), Fraction::zero())]
        ];
    
        let data2: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(8, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(6, 1), Fraction::zero())],
                                        vec![BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(-4, 1), Fraction::zero()), BigM::new(Fraction::new(5, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero())]
                                        ];

        let columns2: Vec<String> = vec!["x".to_string(), "y".to_string(), "z".to_string(), "r".to_string(), "s".to_string(), "t".to_string(), "Value".to_string()];
        let rows2: Vec<String> = vec!["r".to_string(), "s".to_string(), "t".to_string(), "P".to_string()];
        let row_ans2: Vec<String> = vec!["x".to_string(), "y".to_string(), "t".to_string(), "P".to_string()];

        let solution2: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(16, 7), Fraction::zero()), BigM::new(Fraction::new(2, 7), Fraction::zero()), BigM::new(Fraction::new(3, 7), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(32, 7), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(6, 7), Fraction::zero()), BigM::new(Fraction::new(-1, 7), Fraction::zero()), BigM::new(Fraction::new(2, 7), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(12, 7), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-13, 7), Fraction::zero()), BigM::new(Fraction::new(1, 7), Fraction::zero()), BigM::new(Fraction::new(-2, 7), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(30, 7), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(107, 7), Fraction::zero()), BigM::new(Fraction::new(2, 7), Fraction::zero()), BigM::new(Fraction::new(17, 7), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(144, 7), Fraction::zero())]
        ];
    
        let data3: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(0, 1), Fraction::zero()), BigM::new(Fraction::new(20, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(7, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(-1, 1), Fraction::new(-1, 1)), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(1, 1)), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::new(-4, 1))]
        ];

        let columns3: Vec<String> = vec!["x".to_string(), "y".to_string(), "z".to_string(), "s1".to_string(), "s2".to_string(), "s3".to_string(), "a1".to_string(), "Value".to_string()];
        let rows3: Vec<String> = vec!["s1".to_string(), "s2".to_string(), "a1".to_string(), "P".to_string()];
        let row_ans3: Vec<String> = vec!["z".to_string(), "s2".to_string(), "x".to_string(), "P".to_string()];

        let solution3: Vec<Vec<BigM>> = vec![vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::new(-2, 1), Fraction::zero()), BigM::new(Fraction::new(12, 1), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(3, 1), Fraction::zero()), BigM::new(Fraction::new(-3, 1), Fraction::zero()), BigM::new(Fraction::new(15, 1), Fraction::zero())],
        vec![BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(4, 1), Fraction::zero())],
        vec![BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(2, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::zero(), Fraction::zero()), BigM::new(Fraction::new(1, 1), Fraction::zero()), BigM::new(Fraction::new(-1, 1), Fraction::new(1, 1)), BigM::new(Fraction::new(16, 1), Fraction::zero())]
        ];
    
        let mat1: Matrix = Matrix::new_with_data(3, 5, data1);
        let ans1: Matrix = Matrix::new_with_data(3, 5, solution1);
        let mat2: Matrix = Matrix::new_with_data(4, 7, data2);
        let ans2: Matrix = Matrix::new_with_data(4, 7, solution2);
        let mat3: Matrix = Matrix::new_with_data(4, 8, data3);
        let ans3: Matrix = Matrix::new_with_data(4, 8, solution3);
        // let mat4: Conventional<BigM> = data_into_mat(data4, ());
        // let ans4: Conventional<BigM> = data_into_mat(solution4, ());

        let (simplex_ans1, simplex_rows1) = simplex(mat1, rows1, &columns1);
        let (simplex_ans2, simplex_rows2) = simplex(mat2, rows2, &columns2);
        let (simplex_ans3, simplex_rows3) = simplex(mat3, rows3, &columns3);

        assert_eq!(simplex_ans1, ans1);
        assert_eq!(simplex_ans2, ans2);
        assert_eq!(simplex_ans3, ans3);
        assert_eq!(simplex_rows1, row_ans1);
        assert_eq!(simplex_rows2, row_ans2);
        assert_eq!(simplex_rows3, row_ans3);
        // assert_eq!(simplex(mat4), ans4);

    }
}