#ifndef TA_H
#define TA_H

#include <tee_internal_api.h>
	
	
#define TA_AES_ALGO_ECB			0
#define TA_AES_ALGO_CBC			1
#define TA_AES_ALGO_GCM			2

#define TA_AES_SIZE_128BIT		(128 / 8)
#define TA_AES_SIZE_256BIT		(256 / 8)

#define TA_AES_MODE_ENCODE		        1
#define TA_AES_MODE_DECODE		        0

#define AES                     0 // aes-gcm-128
#define SPONGENT                1 // spongent-128


/* The function IDs implemented in this TA */
#define SET_KEY                               0
#define ATTEST                                1
#define HANDLE_INPUT                          2
#define ENTRY                                 3

void handle_output(void *session, uint8_t *num, uint16_t output_id, unsigned char *conn_id,
					 unsigned char *data, uint32_t data_len, unsigned char *tag);

TEE_Result handle_input(void *session, uint32_t param_types, TEE_Param params[4]);

#endif 
