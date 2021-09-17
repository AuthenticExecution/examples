#ifndef CONTROLLER_H
#define CONTROLLER_H

#include <tee_internal_api.h>

#define CONTROLLER_UUID \
	{ 0xf8805eb7, 0x14c6, 0x49bd, { 0x86, 0xe2, 0xe7, 0x71, 0xca, 0xfd, 0x9a, 0x2f} }



TEE_Result entry(void *session, uint32_t param_types, TEE_Param params[4]);
void find_input_func(void *session, uint32_t param_types, TEE_Param params[4],
										uint16_t io_id, unsigned char* data, uint32_t size);

//sm_input
void button_pressed(void *session, uint32_t param_types, TEE_Param params[4],
									unsigned char* data_input, uint32_t data_len);
//sm_output
void toggle_led(void *session, uint8_t *num, unsigned char *conn_id, unsigned char *data,
									 uint32_t data_len, unsigned char *tag);
void increment_presses(void *session, uint8_t *num, unsigned char *conn_id, unsigned char *data,
									 uint32_t data_len, unsigned char *tag);

#endif 
