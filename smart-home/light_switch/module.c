#include <sancus/reactive.h>

SM_DATA({name}) uint16_t switch_on = 0;

SM_OUTPUT({name}, send_switch_state);

SM_INPUT({name}, set_switch, data, len) {
    if(len < 2) {
        return;
    }

    switch_on = *((uint16_t *) data);
    send_switch_state(data, len);
}