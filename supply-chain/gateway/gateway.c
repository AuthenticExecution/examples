SM_OUTPUT(fw_start_shipment);
SM_OUTPUT(fw_end_shipment);
SM_OUTPUT(fw_send_sensor_data);

SM_INPUT(rcv_start_shipment, data, data_len) {
	OUTPUT(fw_start_shipment, data, data_len);
}

SM_INPUT(rcv_end_shipment, data, data_len) {
	OUTPUT(fw_end_shipment, data, data_len);
}

SM_INPUT(rcv_send_sensor_data, data, data_len) {
	OUTPUT(fw_send_sensor_data, data, data_len);
}