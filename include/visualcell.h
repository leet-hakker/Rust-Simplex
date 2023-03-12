#include <stdlib.h>

extern const int BORDERPX;
extern const int CELL_W_TO_H_RATIO;
// extern const int CELLWIDTH;
// extern const int CELLHEIGHT;

typedef struct VisualCell {
  int x;
  int y;
  int x_dim;
  int y_dim;
  bool selected;
  uint16_t bgcol;
  char *contents;
} VisualCell;

VisualCell *new_viscell(int x, int y, int x_dim, int y_dim, bool selected, char *contents);
void free_viscell(VisualCell *cell);
void draw_cell(VisualCell *cell);