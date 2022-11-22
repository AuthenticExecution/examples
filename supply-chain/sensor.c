#include <sancus/reactive.h>

#include <stdio.h>

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
  puts("[{name}] starting sensing");
  send_sensor_data(data, len);
}