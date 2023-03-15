#include <gint/keyboard.h>
#include <gint/keycodes.h>
#include <stdio.h>
#include <string.h>
#include "../include/tableux.h"
#include <gint/display.h>


void process_cursor_movement(key_event_t key, uint8_t *cursor_x, uint8_t *cursor_y, Tableux *tab) {
    switch (key.key) {
        // Up
        case KEY_UP:
            if (*cursor_y > 0) {
                *cursor_y -= 1;
            }
            return;
            break;
        // Down
        case KEY_DOWN:
            if (*cursor_y < tab->rows-1) {
                *cursor_y += 1;
            }
            return;
            break;
        // Left
        case KEY_LEFT:
            if (*cursor_x > 0) {
                *cursor_x -= 1;
            }
            return;
            break;
        // Right
        case KEY_RIGHT:
            if (*cursor_x < tab->columns-1) {
                *cursor_x += 1;
            }
            return;
            break;
        
        default:
            break;
    }
}


void process_common_keys(key_event_t key, uint8_t *cursor_x, uint8_t *cursor_y, Tableux *tab) {
    if (keycode_digit(key.key) > 0) {
        char *prev_val = tab->grid[*cursor_y][*cursor_x]->contents;
        sprintf(tab->grid[*cursor_y][*cursor_x]->contents, "%s%d", prev_val, keycode_digit(key.key));
    } else if (key.key == KEY_0) {
        // Work-around because keycode_digit() for some reason does not include 0.
        char *prev_val = tab->grid[*cursor_y][*cursor_x]->contents;
        sprintf(tab->grid[*cursor_y][*cursor_x]->contents, "%s0", prev_val);
    } else if (key.key == KEY_DEL) {
        const unsigned int length = strlen(tab->grid[*cursor_y][*cursor_x]->contents);
        if (length > 0) tab->grid[*cursor_y][*cursor_x]->contents[length-1] = '\0';
    } else if (key.key == KEY_MINUS) {
        const unsigned int length = strlen(tab->grid[*cursor_y][*cursor_x]->contents);
        if (length == 0) tab->grid[*cursor_y][*cursor_x]->contents[0] = '-';
    }
}


void process_key_rowcol_stage(key_event_t key, uint8_t *cursor_x, uint8_t *cursor_y, Tableux *tab) {
    // Code for EXE is common in both branches,
    // so process it first to possibly exit early.
    
    if (key.key == KEY_EXE) {
        // If the currently selected cell is editable,
        // switch the mode of editing.
        if (tab->grid[*cursor_y][*cursor_x]->editable) {
            tab->grid[*cursor_y][*cursor_x]->editing = !tab->grid[*cursor_y][*cursor_x]->editing;
        }
        return;
    }

    // Processing inputs differently for when we
    // are editing a cell than not.
    //
    // First branch is when not editing a cell.
    if (!tab->grid[*cursor_y][*cursor_x]->editing) {
        process_cursor_movement(key, cursor_x, cursor_y, tab);
    // When editing a cell
    } else if (key.alpha) {
        char *ch;
        switch (key.key) {
            case KEY_XOT:
                ch = "A";
                break;
            
            case KEY_LOG:
                ch = "B";
                break;

            case KEY_LN:
                ch = "C";
                break;
            
            case KEY_SIN:
                ch = "D";
                break;
            
            case KEY_COS:
                ch = "E";
                break;
            
            case KEY_TAN:
                ch = "F";
                break;
            
            case KEY_FRAC:
                ch = "G";
                break;
            
            case KEY_FD:
                ch = "H";
                break;
            
            case KEY_LEFTP:
                ch = "I";
                break;
            
            case KEY_RIGHTP:
                ch = "J";
                break;
            
            case KEY_COMMA:
                ch = "K";
                break;
            
            case KEY_ARROW:
                ch = "L";
                break;
            
            case KEY_7:
                ch = "M";
                break;
            
            case KEY_8:
                ch = "N";
                break;
            
            case KEY_9:
                ch = "O";
                break;
            
            case KEY_4:
                ch = "P";
                break;
            
            case KEY_5:
                ch = "Q";
                break;
            
            case KEY_6:
                ch = "R";
                break;
            
            case KEY_MUL:
                ch = "S";
                break;
            
            case KEY_DIV:
                ch = "T";
                break;
            
            case KEY_1:
                ch = "U";
                break;
            
            case KEY_2:
                ch = "V";
                break;
            
            case KEY_3:
                ch = "W";
                break;
            
            case KEY_ADD:
                ch = "X";
                break;

            case KEY_SUB:
                ch = "Y";
                break;
            
            case KEY_0:
                ch = "Z";
                break;

            default:
                ch = "";
                break;
            }
            char *prev_val = tab->grid[*cursor_y][*cursor_x]->contents;
            sprintf(tab->grid[*cursor_y][*cursor_x]->contents, "%s%s", prev_val, ch);
    } else {
        process_common_keys(key, cursor_x, cursor_y, tab);
    }
}

void process_key_tableux_stage(key_event_t key, uint8_t *cursor_x, uint8_t *cursor_y, Tableux *tab) {
    // Code for EXE is common in both branches,
    // so process it first to possibly exit early.
    
    if (key.key == KEY_EXE) {
        // If the currently selected cell is editable,
        // switch the mode of editing.
        if (tab->grid[*cursor_y][*cursor_x]->editable) {
            tab->grid[*cursor_y][*cursor_x]->editing = !tab->grid[*cursor_y][*cursor_x]->editing;
        }
        return;
    }

    // Processing inputs differently for when we
    // are editing a cell than not.
    //
    // First branch is when not editing a cell.
    if (!tab->grid[*cursor_y][*cursor_x]->editing) {
        process_cursor_movement(key, cursor_x, cursor_y, tab);
    // When editing a cell
    } else {
        process_common_keys(key, cursor_x, cursor_y, tab);
    }
}