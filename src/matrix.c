#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include "../include/m.h"


typedef struct Matrix {
    int rows;
    int columns;
    BigM **values;
} Matrix;

void free_matrix(Matrix *mat) {
    if (mat == NULL) {
        return;
    }
    for (int i=0; i < mat->rows; i++) {
        for (int j=0; j < mat->columns; j++) {
            if (&(mat->values[i]) != NULL && &(mat->values[i][j]) != NULL) {
                free(&(mat->values[i][j]));
            }
        }
        free(mat->values[i]);
    }

    free(mat);
}

Matrix* new_matrix(int rows, int columns) {
    Matrix* mat = malloc(sizeof(Matrix));
    if (mat == NULL) {
        return NULL;
    }
    mat->rows = rows;
    mat->columns = columns;
    mat->values = malloc(sizeof(BigM**) * rows);
    if (mat->values == NULL) {
        free(mat);
        return NULL;
    }
    for (int i = 0; i < rows; i++) {
        mat->values[i] = malloc(sizeof(BigM) * columns);
        if (mat->values[i] == NULL) {
            for (int j = 0; j < i; j++) {
                free(mat->values[j]);
            }
            free(mat->values);
            free(mat);
            return NULL;
        }
    }
    return mat;
}

Matrix* new_matrix_with_data(int rows, int columns, BigM** data) {
    Matrix* mat = new_matrix(rows, columns);
    if (mat == NULL) {
        return NULL;
    }
    for (int i = 0; i < rows; i++) {
        memcpy(mat->values[i], data[i], sizeof(BigM) * columns);
    }
    return mat;
}

BigM get(const Matrix* mat, int row, int column) {
    return mat->values[row][column];
}

bool set(Matrix* mat, int row, int column, BigM value) {
    if (row > mat->rows-1) { 
        return false;
    } else if (column > mat->columns-1) {
        return false;
    }
    mat->values[row][column] = value;
    return true;
}

void print_mat(Matrix *mat) {
    char *str;
    for (int i=0; i < mat->rows; i++) {
        for (int j=0; j < mat->columns; j++) {
            str = BigM_str(get(mat, i, j));
            printf("%s ", str);
            free(str);
        }
        printf("\n");
    }
}