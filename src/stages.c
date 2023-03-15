#include "../include/tableux.h"
#include <stdint.h>
#include <string.h>

uint8_t check_stage_0(uint8_t stage, Tableux *tab) {
    char *empty = calloc(1, 1);
    for (int i=0; i < tab->columns; i++) {
        if (tab->grid[0][i]->contents[0] == empty[0])
            return stage;
    }

    for (int i=0; i < tab->rows; i++) {
        if (tab->grid[i][0]->contents[0] == empty[0])
            return stage;
    }

    free(empty);

    // Change the tableux so that it is ready for
    // stage 1.

    for (int i=0; i < tab->columns; i++) {
        tab->grid[0][i]->editable = false;
        tab->grid[0][i]->editing = false;
    }

    for (int i=0; i < tab->rows; i++) {
        tab->grid[i][0]->editable = false;
        tab->grid[i][0]->editing = false;
    }

    for (int i=1; i < tab->rows; i++) {
        for (int j=1; j < tab->columns; j++) {
            tab->grid[i][j]->editable = true;
        }
    }

    return stage + 1;
}

uint8_t check_stage_1(uint8_t stage, Tableux *tab) {
    char *empty = calloc(1, 1);
    for (int i=1; i < tab->rows; i++) {
        for (int j=1; j < tab->columns; j++) {
            if (tab->grid[i][j]->contents[0] == empty[0]) {
                return stage;
            }
        }
    }

    free(empty);

    for (int i=1; i < tab->rows; i++) {
        for (int j=1; j < tab->columns; j++) {
            tab->grid[i][j]->editable = false;
            tab->grid[i][j]->editing = false;
        }
    }

    return stage+1;

}