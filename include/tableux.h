#include <stdlib.h>
#include <stdbool.h>
#include "visualcell.h"

typedef struct Tableux {
    uint8_t rows;
    uint8_t columns;
    uint8_t cursor_x;
    uint8_t cursor_y;
    VisualCell ***grid;
} Tableux;

Tableux *new_tableux(int rows, int columns);
void free_tableux(Tableux *tab);
void display_tableux(Tableux *tab);
void update_cursor(uint8_t cursor_x, uint8_t cursor_y, Tableux *tab);