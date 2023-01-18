SM_OUTPUT(fw_int0);
SM_OUTPUT(fw_int1);

SM_INPUT(send_int0, data, data_len) {
    DMSG("Sending packet to interface 0");
	fw_int0(data, data_len);
}

SM_INPUT(send_int1, data, data_len) {
    DMSG("Sending packet to interface 1");
	fw_int1(data, data_len);
}