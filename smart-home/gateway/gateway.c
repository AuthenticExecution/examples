typedef struct status {
	float desired_temp;
	float actual_temp;
	uint16_t auto_heating;
	uint16_t heating_on;
	uint16_t switch_on; 
} Status;

Status status = { 0.0, 0.0, 0, 0, 0 };
void send_status_info(void);

SM_OUTPUT(set_heating);
SM_OUTPUT(set_switch);
SM_OUTPUT(send_status);

SM_ENTRY(check_heater, data, data_len) {
	(void) data;
	(void) data_len;

	if(status.auto_heating) {
		uint16_t desired_state = status.actual_temp < status.desired_temp;

		if(desired_state != status.heating_on) {
			DMSG("Setting thermostat to: %d", desired_state);
			set_heating((unsigned char *) &desired_state, 2);
		}
	}

	send_status_info();
}

/* received from thermostat */
SM_INPUT(set_heating_state, data, data_len) {
	if(data_len < 2) {
		EMSG("set_heating_state: invalid data");
		return;
	}

	TEE_MemMove(&status.heating_on, data, 2);
	send_status_info();
}

/* received from temp sensor */
SM_INPUT(set_actual_temp, data, data_len) {
	if(data_len < 2) {
		EMSG("set_actual_temp: invalid data");
		return;
	}

	uint16_t temp;
	TEE_MemMove(&temp, data, 2);
	status.actual_temp = temp / 10.0; // received tmp is multiplied by 10
	send_status_info();
}

/* received from light switch */
SM_INPUT(set_switch_state, data, data_len) {
	if(data_len < 2) {
		EMSG("set_switch_state: invalid data");
		return;
	}

	TEE_MemMove(&status.switch_on, data, 2);
	send_status_info();
}

/* received from web */
SM_INPUT(set_desired_temp, data, data_len) {
	if(data_len < 4) {
		EMSG("set_desired_temp: invalid data");
		return;
	}

	TEE_MemMove(&status.desired_temp, data, 4);
	status.auto_heating = 1;
	send_status_info();
}

SM_INPUT(enable_heating, data, data_len) {
	if(data_len < 2) {
		EMSG("enable_heating: invalid data");
		return;
	}

	status.auto_heating = 0;
	set_heating(data, data_len);
	send_status_info();
}

SM_INPUT(enable_switch, data, data_len) {
	if(data_len < 2) {
		EMSG("enable_switch: invalid data");
		return;
	}

	set_switch(data, data_len);
}

void send_status_info(void) {
	const int json_size = 256; // 256 bytes should be enough.
	char json[json_size];

	int nchars = snprintf(
		json, 
		json_size,
		"{"
		"\"desired_temp\":%d.%02u,"
		"\"actual_temp\":%d.%02u,"
		"\"auto_heating\":%s,"
		"\"heating_on\":%s,"
		"\"switch_on\":%s"
		"}",
		(int) status.desired_temp, (int) ((status.desired_temp - (int) status.desired_temp ) * 100),
		(int) status.actual_temp, (int) ((status.actual_temp - (int) status.actual_temp ) * 100),
		status.auto_heating == 0 ? "false" : "true",
		status.heating_on == 0 ? "false" : "true",
		status.switch_on == 0 ? "false" : "true"
	);

	if(nchars < 0) {
		EMSG("Failed to create JSON string");
		return;
	}

	send_status((unsigned char *) json, nchars);
}