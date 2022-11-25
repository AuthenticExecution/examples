#include <sancus/reactive.h>

#include <stdio.h>

// sensor data
SM_DATA({name}) uint16_t sensor_id = 123;
SM_DATA({name}) uint16_t shipment_id = 56;
SM_DATA({name}) uint16_t data_id = 32;

SM_OUTPUT({name}, start_shipment);
SM_OUTPUT({name}, end_shipment);

SM_ENTRY({name}) void trigger_start_shipment(uint8_t* data, size_t len) {
    uint16_t msg[3] = { sensor_id, ++shipment_id, data_id };
    start_shipment((uint8_t *) msg, 6);
}

SM_ENTRY({name}) void trigger_end_shipment(uint8_t* data, size_t len) {
    uint16_t msg[3] = { sensor_id, shipment_id, data_id };
    end_shipment((uint8_t *) msg, 6);
}