#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

#include "../include/visualcell.h"

const uint16_t ROWCOLBGCOLOUR = 0xdefb;

typedef struct Tableux {
    uint8_t rows;
    uint8_t columns;
    uint8_t cursor_x;
    uint8_t cursor_y;
    VisualCell ***grid;
} Tableux;

Tableux *new_tableux(uint8_t rows, uint8_t columns) {
    Tableux *tab = malloc(sizeof(Tableux));
    tab->rows = rows;
    tab->columns = columns;

    // To ensure that we fill the screen.
    unsigned int cellwidth = 384 / columns;
    unsigned int cellheight = cellwidth / CELL_W_TO_H_RATIO; 

    // Using calloc to allow us to not set some values.
    tab->grid = calloc(rows, sizeof(VisualCell**));
    for (int i = 0; i < rows; i++) {
        tab->grid[i] = calloc(columns+1, sizeof(VisualCell*));
    }

    // Assigning each member of the grid its own cell.
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < columns; j++) {
            tab->grid[i][j] = new_viscell(j*cellwidth+BORDERPX, i*cellheight+BORDERPX, cellwidth, cellheight, false, "");
        }
    }

    // Setting the first row and column to have a different background colour.
    for (int i=0; i < rows; i++) {
        tab->grid[i][0]->bgcol = ROWCOLBGCOLOUR;
    }

    for (int i=0; i < columns; i++) {
        tab->grid[0][i]->bgcol = ROWCOLBGCOLOUR;
    }

    // Setting the cursor
    tab->cursor_x = 1;
    tab->cursor_y = 0;

    tab->grid[tab->cursor_y][tab->cursor_x]->selected = true;

    // Important cell texts that won't change
    tab->grid[0][0]->contents = "B.v.";
    tab->grid[0][columns-1]->contents = "Value";

    return tab;
}

void free_tableux(Tableux *tab) {
    if (tab == NULL) {
        return;
    }
    for (int i=0; i < tab->rows; i++) {
        for (int j=0; j < tab->columns; j++) {
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

void update_cursor(uint8_t cursor_x, uint8_t cursor_y, Tableux *tab) {
    tab->grid[tab->cursor_y][tab->cursor_x]->selected = false;
    tab->grid[cursor_y][cursor_x]->selected = true;
    tab->cursor_x = cursor_x;
    tab->cursor_y = cursor_y;
}