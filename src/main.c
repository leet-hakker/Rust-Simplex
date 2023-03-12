#include <gint/display.h>
#include <gint/keyboard.h>
#include <stdio.h>
#include "../include/tableux.h"
#include "../include/input.h"


int main() {
	key_event_t key;
	uint8_t cursor_x = 1;
	uint8_t cursor_y = 0;

	dclear(0xffff);

	// char **row_names = ("r", "s", "P");
	// char **column_names = ("x", "y", "r", "s");

	Tableux *tab = new_tableux(4, 6);

	while (true) {
		display_tableux(tab);

		dupdate();

		dclear(0xffff);

		key = getkey();

		if (key.key == 0x84)
			break;

		if (process_key(key) <= 3) {
			switch (key.key) {
				// Up
				case 0x86:
					if (cursor_y > 0) {
						cursor_y -= 1;
					}
					break;
				// Down
				case 0x75:
					if (cursor_y < tab->rows-1) {
						cursor_y += 1;
					}
					break;
				// Left
				case 0x85:
					if (cursor_x > 0) {
						cursor_x -= 1;
					}
					break;
				// Right
				case 0x76:
					if (cursor_x < tab->columns-1) {
						cursor_x += 1;
					}
					break;
			}
		}

		update_cursor(cursor_x, cursor_y, tab);
	}

	free_tableux(tab);

	return 0;
}

