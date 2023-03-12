#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <gint/display.h>
#include <gint/display-cg.h>

const int BORDERPX = 2;
const int CELLWIDTH = 50;
const int CELLHEIGHT = 25;

typedef struct VisualCell {
    int x;
    int y;
    int x_dim;
    int y_dim;
    bool selected;
    char *contents;
} VisualCell;

VisualCell *new_viscell(int x, int y, int x_dim, int y_dim, bool selected, char *contents) {
    VisualCell *cell = malloc(sizeof(VisualCell));

    cell->x = x;
    cell->y = y;
    cell->x_dim = x_dim;
    cell->y_dim = y_dim;
    cell->selected = false;
    cell->contents = (char*)calloc(6, 1);
    strcpy(cell->contents, contents);

    return cell;
}

void free_viscell(VisualCell *cell) {
    free(cell->contents);
    free(cell);
}

void draw_cell(VisualCell *cell) {
    if (cell->selected) {
        drect_border(cell->x, cell->y, cell->x + cell->x_dim, cell->y + cell->y_dim, C_WHITE, 3, C_DARK);
    } else {
        drect_border(cell->x, cell->y, cell->x + cell->x_dim, cell->y + cell->y_dim, C_WHITE, 3, C_LIGHT);
    }

    dtext(cell->x+BORDERPX*2, cell->y+BORDERPX*2, C_BLACK, cell->contents);
}