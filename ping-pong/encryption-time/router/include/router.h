#ifndef ROUTER_H
#define ROUTER_H

#include <tee_internal_api.h>

#define ROUTER_UUID \
	{ 0x83cd2a12, 0xf892, 0x4d68, { 0xbc, 0x06, 0x0f, 0xba, 0x22, 0x28, 0x6e, 0x41} }


TEE_Result entry(void *session, uint32_t param_types, TEE_Param params[4]);
void find_input_func(void *session, uint32_t param_types, TEE_Param params[4],
										uint16_t io_id, unsigned char* data, uint32_t size);

//sm_input
void send_int0(void *session, uint32_t param_types, TEE_Param params[4],
									unsigned char* data_input, uint32_t data_len);
void send_int1(void *session, uint32_t param_types, TEE_Param params[4],
									unsigned char* data_input, uint32_t data_len);
//sm_output
void fw_int0(void *session, uint8_t *num, unsigned char *conn_id, unsigned char *data,
									 uint32_t data_len, unsigned char *tag);
void fw_int1(void *session, uint8_t *num, unsigned char *conn_id, unsigned char *data,
									 uint32_t data_len, unsigned char *tag);

#endif 
