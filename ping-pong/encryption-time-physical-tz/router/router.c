SM_OUTPUT(fw_int0);

SM_INPUT(send_int0, data, data_len) {
    DMSG("Sending packet to interface 0\n");
	OUTPUT(fw_int0, data, data_len);
}