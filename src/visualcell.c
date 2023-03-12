#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <gint/display.h>
#include <gint/display-cg.h>

const int BORDERPX = 1;
const int CELL_W_TO_H_RATIO = 2;
// const int CELLWIDTH = 50;
// const int CELLHEIGHT = 25;

typedef struct VisualCell {
    int x;
    int y;
    int x_dim;
    int y_dim;
    bool selected;
    uint16_t bgcol;
    char *contents;
} VisualCell;

VisualCell *new_viscell(int x, int y, int x_dim, int y_dim, bool selected, char *contents) {
    VisualCell *cell = malloc(sizeof(VisualCell));

    cell->x = x;
    cell->y = y;
    cell->x_dim = x_dim;
    cell->y_dim = y_dim;
    cell->selected = selected;
    cell->bgcol = 0xffff;
    cell->contents = (char*)calloc(6, 1);
    strcpy(cell->contents, contents);

    return cell;
}

void free_viscell(VisualCell *cell) {
    free(cell->contents);
    free(cell);
}

void draw_cell(VisualCell *cell) {
    drect_border(cell->x, cell->y, cell->x + cell->x_dim, cell->y + cell->y_dim, cell->bgcol, BORDERPX+2*(cell->selected), 0x0000); // 0xad55

    dtext(cell->x+BORDERPX*2+2, cell->y+BORDERPX*2, 0x0000, cell->contents);
}