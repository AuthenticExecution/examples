#include <sancus/reactive.h>

#include <stdio.h>
#include <stdlib.h>

#define PART_SIZE 128

SM_OUTPUT({name}, start_shipment);
SM_OUTPUT({name}, end_shipment);
SM_OUTPUT({name}, send_sensor_data);

SM_ENTRY({name}) void trigger_start_shipment(uint8_t* data, size_t len) {
    puts("[{name}] starting shipment");
    start_shipment(data, len);
}

SM_ENTRY({name}) void trigger_end_shipment(uint8_t* data, size_t len) {
    puts("[{name}] ending shipment");
    end_shipment(data, len);
}

SM_ENTRY({name}) void trigger_start_sensing(uint8_t* data, size_t len) {
    int i = 0;
    uint8_t sensor_data[PART_SIZE]; // buffer that will contain sensor data

    puts("[{name}] starting sensing");

    if(len != 2) {
      puts("Bad data received");
      return;
    }

    // data_size: total size of sensor data, in kilobytes
    int data_size = (data[0] << 8) | data[1];
    if(data_size == 0) {
      puts("Data size is zero");
      return;
    }

    // compute number of parts of data, in bytes
    int num_parts = data_size * 1024 / PART_SIZE;

    // notify that sensing has started
    send_sensor_data((uint8_t *) &num_parts, 2);

    // send sensor data, one part at a time
    for(i=num_parts; i>0; i--) {
      send_sensor_data(sensor_data, PART_SIZE);
    }

    // notify that sensing has ended. i should be equal to zero
    send_sensor_data((uint8_t *) &i, 2);
}