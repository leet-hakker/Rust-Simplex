#ifndef INPUT_H
#define INPUT_H
#include <gint/keyboard.h>

void process_key_rowcol_stage(key_event_t key, uint8_t *cursor_x, uint8_t *cursor_y, Tableux *tab);
void process_key_tableux_stage(key_event_t key, uint8_t *cursor_x, uint8_t *cursor_y, Tableux *tab);
#endif