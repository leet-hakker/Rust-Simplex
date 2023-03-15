#include <gint/display.h>
#include <gint/keyboard.h>
#include <stdio.h>
#include "../include/fraction.h"
#include "../include/m.h"
#include "../include/matrix.h"
#include "../include/simplex.h"
#include "../include/tableux.h"
#include "../include/input.h"
#include "../include/stages.h"


int tableux_input_stage(Tableux *tab) {
	key_event_t key;
	uint8_t cursor_x = 1;
	uint8_t cursor_y = 0;
	uint8_t stage = 0;

	while (true) {
		display_tableux(tab);

		dupdate();

		dclear(0xffff);

		key = getkey();

		if (key.key == 0x84)
			break;

		switch (stage)
		{
		case 0:
			process_key_rowcol_stage(key, &cursor_x, &cursor_y, tab);
			stage = check_stage_0(stage, tab);
			break;
		
		case 1:
			process_key_tableux_stage(key, &cursor_x, &cursor_y, tab);
			stage = check_stage_1(stage, tab);
			break;

		default:
			break;
		}

		// Once `stage` reaches 2, the tableux is ready.
		// Move on to the next part of the program.
		if (stage == 2) {
			return 0;
		}

		update_cursor(cursor_x, cursor_y, tab);
	}

	return 0;
}


int solve_stage(Tableux *tab) {
	BigM **data = extract_grid(tab);
	char **row_names = extract_row_names(tab);
	char **column_names = extract_column_names(tab);

	Matrix *mat = new_matrix_with_data(tab->rows-1, tab->columns-1, data);
	*mat = simplex(*mat, row_names, column_names);

	tab = matrix_into_tableux(mat, row_names, tab);

	return 0;
}


int main() {
	dclear(0xffff);

	// char **row_names = ("r", "s", "P");
	// char **column_names = ("x", "y", "r", "s");

	Tableux *tab = new_tableux(4, 6);

	tableux_input_stage(tab);
	solve_stage(tab);
	display_tableux(tab);
	dupdate();
	getkey();
	getkey();
	getkey();

	free_tableux(tab);

	return 0;
}

