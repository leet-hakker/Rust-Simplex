use crate::m::BigM;
use crate::fraction::Fraction;
use matrix::prelude::*;

fn isoptimal(matrix: Compressed, objective_row: i8) -> bool {
}

fn simplex(matrix: Compressed, column_names: [&str], row_names: [&str], objective_row: i8, two_stage: bool) {

}

// def simplex(matrix,
//             column_names,
//             row_names,
//             objective_row=-1,
//             two_stage=False,
//             show_changes=False):

//     while not isoptimal(matrix, objective_row=objective_row):
//         column = pivot_column(matrix, objective_row=objective_row)
//         row = pivot_row(matrix, column, two_stage=two_stage)


//         matrix.rowmul(row, 1 / matrix[column, row])

//         row_names[row] = column_names[column]

//         for i in range(matrix.dims[1]):
//             if i == row:
//                 continue


//             row_modifier = matrix.row(row) * (-matrix[column, i])


//             matrix.rowadd(i, row_modifier)

//         if show_changes:
//             yield matrix, row_names, (column, row)

//     if not show_changes:
//         yield matrix, row_names, (-1, -1)