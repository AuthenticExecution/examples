#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <router.h>
#include <authentic_execution.h>

//sm_output
void fw_int0(void *session, uint8_t *num, unsigned char *conn_id, unsigned char *data,
									 uint32_t data_len, unsigned char *tag){

	uint16_t output_id = 2;
	handle_output(session, num, output_id, conn_id, data, data_len, tag);
}

//sm_output
void fw_int1(void *session, uint8_t *num, unsigned char *conn_id, unsigned char *data,
									 uint32_t data_len, unsigned char *tag){

	uint16_t output_id = 3;
	handle_output(session, num, output_id, conn_id, data, data_len, tag);
}

// sm_input
void send_int0(void *session, uint32_t param_types, TEE_Param params[4],
										unsigned char* data_input, uint32_t data_len) {

	TEE_Time t = { };
	uint8_t i = 0;
	uint8_t num = 0;
	uint8_t index = 0;
	unsigned char *data;
	unsigned char *conn_id;
	unsigned char *tag;
	conn_id = malloc(16 * 2);
	data = malloc(16 * 16); /* Maximum number of output*/
	tag = malloc(16 * 16);
    //--------------------------------
	memcpy(data, data_input, data_len);

	// Data must be forwarded to interface 0
    DMSG("Sending packet to interface 0\n");
    fw_int0(session, &num, conn_id, data, data_len, tag); //sm_output
	
	
	TEE_MemMove(params[1].memref.buffer, conn_id, (2 * num));	
	TEE_MemMove(params[2].memref.buffer, data, ((data_len * num) + num));
	TEE_MemMove(params[3].memref.buffer, tag, (16 * num));
    //-------------------------------------------------------------------------------
	params[0].value.b = i + num;
	
	free(data);
	free(conn_id);
	free(tag);

	TEE_GetREETime(&t);
	printf("time at the end of router TA(send_int0): %u.%03u\n", (unsigned int)t.seconds,
	       (unsigned int)t.millis);
}

// sm_input
void send_int1(void *session, uint32_t param_types, TEE_Param params[4],
										unsigned char* data_input, uint32_t data_len) {

	TEE_Time t = { };
	uint8_t i = 0;
	uint8_t num = 0;
	uint8_t index = 0;
	unsigned char *data;
	unsigned char *conn_id;
	unsigned char *tag;
	conn_id = malloc(16 * 2);
	data = malloc(16 * 16); /* Maximum number of output*/
	tag = malloc(16 * 16);
    //--------------------------------
	memcpy(data, data_input, data_len);

	// Data must be forwarded to interface 1
    DMSG("Sending packet to interface 1\n");
    fw_int1(session, &num, conn_id, data, data_len, tag); //sm_output
	
	
	TEE_MemMove(params[1].memref.buffer, conn_id, (2 * num));	
	TEE_MemMove(params[2].memref.buffer, data, ((data_len * num) + num));
	TEE_MemMove(params[3].memref.buffer, tag, (16 * num));
    //-------------------------------------------------------------------------------
	params[0].value.b = i + num;
	
	free(data);
	free(conn_id);
	free(tag);

	TEE_GetREETime(&t);
	printf("time at the end of router TA(send_int1): %u.%03u\n", (unsigned int)t.seconds,
	       (unsigned int)t.millis);
}

void find_input_func(void *session, uint32_t param_types, TEE_Param params[4],
									uint16_t io_id, unsigned char* data, uint32_t size){

	switch (io_id)
	{
		case 0:
			send_int0(session, param_types, params, data, size);
		  	break;
		
		case 1:
			send_int1(session, param_types, params, data, size);
		  	break;

	  	default:
		  	break;
	}
}
//sm_entry
TEE_Result entry(void *session, uint32_t param_types, TEE_Param params[4]){
	
	return TEE_SUCCESS;
}



