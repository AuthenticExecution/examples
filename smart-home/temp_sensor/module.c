#include <sancus/reactive.h>

// Note: temperature is multiplied by 10 in order for it to be an integer
#define MIN_TEMP 140
#define MAX_TEMP 300

SM_DATA({name}) uint16_t heating_on = 0;
SM_DATA({name}) uint16_t temperature = 180; 

SM_OUTPUT({name}, send_actual_temp);

SM_ENTRY({name}) void read_from_sensor(uint8_t* data, size_t len) {
    if(heating_on && temperature < MAX_TEMP) {
        temperature += 1;
    }
    else if(!heating_on && temperature > MIN_TEMP) {
        temperature -= 1;
    }

    send_actual_temp((uint8_t *) &temperature, 2);
}


SM_INPUT({name}, set_heating_state, data, len) {
    if(len < 2) {
        return;
    }

    heating_on = *((uint16_t *) data);
}