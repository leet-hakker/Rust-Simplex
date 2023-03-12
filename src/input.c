#include <gint/keyboard.h>
#include <gint/keycodes.h>

// 0: Up
// 1: Down
// 2: Left
// 3: Right
// 4: Cell edit change
// 5: Nothing we care about
uint8_t process_key(key_event_t key) {
    switch (key.key) {
        // up
        case 0x86:
            return 0;
            break;
        
        // down
        case 0x75:
            return 1;
            break;
        
        // left
        case 0x85:
            return 2;
            break;
        
        // right
        case 0x76:
            return 3;
            break;
        
        // EXE
        case 0x15:
            return 4;
            break;
        
        default:
            return 5;
            break;
    }
}