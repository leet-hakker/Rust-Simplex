// #include <gint/display.h>
// #include <gint/keyboard.h>
#include "../include/tableux.h"


int main() {
	// dclear(C_WHITE);

	// char **row_names = ("r", "s", "P");
	// char **column_names = ("x", "y", "r", "s");

	Tableux *tab = new_tableux(3, 5);
	// display_tableux(tab);

	// dupdate();

	// getkey();

	free_tableux(tab);

	return 0;
}

