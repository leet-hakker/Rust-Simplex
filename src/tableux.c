#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

#include "../include/visualcell.h"

typedef struct Tableux {
    uint8_t rows;
    uint8_t columns;
    VisualCell ***grid;
} Tableux;

Tableux *new_tableux(uint8_t rows, uint8_t columns) {
    Tableux *tab = malloc(sizeof(Tableux));
    tab->rows = rows;
    tab->columns = columns;

    tab->grid = calloc(rows+1, sizeof(VisualCell**));
    for (int i = 0; i < rows+1; i++) {
        tab->grid[i] = calloc(columns+1, sizeof(VisualCell*));
    }

    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < columns; j++) {
            tab->grid[i][j] = new_viscell(j*CELLWIDTH+BORDERPX, i*CELLHEIGHT+BORDERPX, CELLWIDTH, CELLHEIGHT, false, "");
        }
    }

    // char *temp = "B.v.";
    tab->grid[0][0]->contents = "B.v.";
    // printf("%s\n", tab->grid[0][0]);
    // printf("%ld", sizeof(VisualCell**)*(rows+1));
    tab->grid[0][columns]->contents = "Value";

    return tab;
}

void free_tableux(Tableux *tab) {
    if (tab == NULL) {
        return;
    }
    for (int i=0; i < tab->rows+1; i++) {
        for (int j=0; j < tab->columns+1; j++) {
            if (tab->grid[i] != NULL && tab->grid[i][j] != NULL) {
                free_viscell(tab->grid[i][j]);
            }
        }
        free(tab->grid[i]);
    }
    free(tab->grid);
    free(tab);
}

void display_tableux(Tableux *tab) {
    for (int i=0; i < tab->rows; i++) {
        for (int j=0; j < tab->columns; j++) {
            draw_cell(tab->grid[i][j]);
        }
    }
}