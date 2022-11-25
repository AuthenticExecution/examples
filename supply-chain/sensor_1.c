#include <sancus/reactive.h>

#include <stdio.h>

#define PART_SIZE 64

// sensor data
SM_DATA({name}) uint16_t sensor_id = 123;
SM_DATA({name}) uint16_t shipment_id = 56;
SM_DATA({name}) uint16_t data_id = 32;

SM_OUTPUT({name}, start_sensing);
SM_OUTPUT({name}, end_sensing);
SM_OUTPUT({name}, send_sensor_data);

SM_ENTRY({name}) void trigger_start_sensing(uint8_t* data, size_t len) {
    uint16_t i = 0;
    uint8_t sensor_data[PART_SIZE]; // buffer that will contain sensor data

    //puts("[{name}] starting sensing");

    if(len != 2) {
      puts("Bad data received");
      return;
    }

    // data_size: total size of sensor data, in kilobytes
    uint16_t data_size = (data[0] << 8) | data[1];
    if(data_size == 0) {
      puts("Data size is zero");
      return;
    }

    // start sensing
    uint16_t msg[3] = { sensor_id, shipment_id, ++data_id };
    start_sensing((uint8_t *) msg, 6);

    // compute number of parts of data, in bytes
    uint16_t num_parts = data_size * 1024 / PART_SIZE;

    // send sensor data, one part at a time
    for(i=num_parts; i>0; i--) {
      send_sensor_data(sensor_data, PART_SIZE);
    }

    // end sensing
    end_sensing(msg, 6);
}