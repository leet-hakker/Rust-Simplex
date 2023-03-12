#include <stdlib.h>
#include <stdbool.h>
#include "visualcell.h"

typedef struct Tableux {
    int rows;
    int columns;
    char **column_names;
    char **row_names;
    VisualCell **grid;
} Tableux;

Tableux *new_tableux(int rows, int columns);
void free_tableux(Tableux *tab);
void display_tableux(Tableux *tab);