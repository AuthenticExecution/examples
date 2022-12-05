#include <sancus/reactive.h>

#include <stdio.h>

// sensor data
SM_DATA({name}) uint16_t sensor_id = 123;
SM_DATA({name}) uint16_t shipment_id = 56;
SM_DATA({name}) uint16_t data_id = 32;

SM_OUTPUT({name}, start_shipment);
SM_OUTPUT({name}, ack_received);

SM_ENTRY({name}) void start(uint8_t* data, size_t len) {
    uint16_t msg[3] = { sensor_id, ++shipment_id, data_id };
    start_shipment((uint8_t *) msg, 6);
}

SM_INPUT({name}, receive_ack, data, len) {
    //puts("[{name}] received ack");
    ack_received(data, len);
}