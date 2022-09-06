SM_OUTPUT(fw_int0);
SM_OUTPUT(fw_int1);

SM_INPUT(send_int0, data, data_len) {
	TEE_Time t = { };

    DMSG("Sending packet to interface 0\n");
	OUTPUT(fw_int0, data, data_len);

	TEE_GetREETime(&t);
	printf("time at the end of router TA(send_int0): %u.%03u\n", (unsigned int)t.seconds,
	       (unsigned int)t.millis);
}

SM_INPUT(send_int1, data, data_len) {
	TEE_Time t = { };

    DMSG("Sending packet to interface 1\n");
	OUTPUT(fw_int1, data, data_len);

	TEE_GetREETime(&t);
	printf("time at the end of router TA(send_int1): %u.%03u\n", (unsigned int)t.seconds,
	       (unsigned int)t.millis);
}