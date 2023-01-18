#include <sancus/reactive.h>

SM_DATA({name}) uint16_t heating_on = 0;

SM_OUTPUT({name}, send_heating_state);

SM_INPUT({name}, set_heating, data, len) {
    if(len < 2) {
        return;
    }

    heating_on = *((uint16_t *) data);
    send_heating_state(data, len);
}