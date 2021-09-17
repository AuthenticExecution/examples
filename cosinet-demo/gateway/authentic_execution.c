#include <inttypes.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

#include <tee_internal_api.h>

#include <controller.h>
#include <authentic_execution.h>
#include <pta_attestation.h>
#include <spongent.h>

static const TEE_UUID pta_attestation_uuid = ATTESTATION_UUID;

void *malloc_aligned(size_t size) {
  size += size % 2;

  return malloc(size);
}

int total_node = 0;

typedef struct
{
    uint8_t  encryption;
	uint16_t conn_id;
    uint16_t io_id;
    uint16_t nonce;
    unsigned char connection_key[16];
} Connection;

typedef struct Node
{
    Connection connection;
    struct Node* next;
} Node;

static Node* connections_head = NULL;

int connections_add(Connection* connection)
{
   Node* node = malloc_aligned(sizeof(Node));

   if (node == NULL)
      return 0;

   node->connection = *connection;
   node->next = connections_head;
   connections_head = node;
   return 1;
}

Connection* connections_get(uint16_t conn_id)
{
    Node* current = connections_head;

    while (current != NULL) {
        Connection* connection = &current->connection;

        if (connection->conn_id == conn_id) {
            return connection;
        }

        current = current->next;
    }

    return NULL;
}

void find_connections(uint16_t io_id, int *arr, uint8_t *num)
{
    Node* current = connections_head;
    while (current != NULL) {
        Connection* connection = &current->connection;
        if (connection->io_id == io_id) {
            arr[*num] = connection->conn_id;
            *num = *num + 1;
        }

        current = current->next;
    }

}

//===============================================================

char module_key[16] = { 0 };

struct aes_cipher {
	uint32_t algo;			/* AES flavour */
	uint32_t mode;			/* Encode or decode */
	uint32_t key_size;		/* AES key size in byte */
	TEE_OperationHandle op_handle;	/* AES ciphering operation */
	TEE_ObjectHandle key_handle;	/* transient object to load the key */
};

//===============================================================

static TEE_Result ta2tee_algo_id(uint32_t param, uint32_t *algo)
{
	switch (param) {
	case TA_AES_ALGO_ECB:
		*algo = TEE_ALG_AES_ECB_NOPAD;
		return TEE_SUCCESS;
	case TA_AES_ALGO_CBC:
		*algo = TEE_ALG_AES_CBC_NOPAD;
		return TEE_SUCCESS;
	case TA_AES_ALGO_GCM:
		*algo = TEE_ALG_AES_GCM;
		return TEE_SUCCESS;
	default:
		EMSG("Invalid algo %u", param);
		return TEE_ERROR_BAD_PARAMETERS;
	}
}
static TEE_Result ta2tee_key_size(uint32_t param, uint32_t *key_size)
{
	switch (param) {
	case 16:
		*key_size = param;
		return TEE_SUCCESS;
	default:
		EMSG("Invalid key size %u", param);
		return TEE_ERROR_BAD_PARAMETERS;
	}
}
static TEE_Result ta2tee_mode_id(uint32_t param, uint32_t *mode)
{
	switch (param) {
	case TA_AES_MODE_ENCODE:
		*mode = TEE_MODE_ENCRYPT;
		return TEE_SUCCESS;
	case TA_AES_MODE_DECODE:
		*mode = TEE_MODE_DECRYPT;
		return TEE_SUCCESS;
	default:
		EMSG("Invalid mode %u", param);
		return TEE_ERROR_BAD_PARAMETERS;
	}
}

static TEE_Result alloc_resources(void *session, uint32_t algo, uint32_t key_size,
                                    uint32_t mode){

	struct aes_cipher *sess;
	TEE_Attribute attr;
	TEE_Result res;
	char *key;

	/* Get ciphering context from session ID */
	sess = (struct aes_cipher *)session;

	res = ta2tee_algo_id(algo, &sess->algo);
	if (res != TEE_SUCCESS)
		return res;

	res = ta2tee_key_size(key_size, &sess->key_size);
	if (res != TEE_SUCCESS)
		return res;

	res = ta2tee_mode_id(mode, &sess->mode);
	if (res != TEE_SUCCESS)
		return res;

	if (sess->op_handle != TEE_HANDLE_NULL)
		TEE_FreeOperation(sess->op_handle);

	/* Allocate operation: AES/CTR, mode and size from params */
	res = TEE_AllocateOperation(&sess->op_handle,
				    sess->algo,
				    sess->mode,
				    sess->key_size * 8);
	if (res != TEE_SUCCESS) {
		EMSG("Failed to allocate operation");
		sess->op_handle = TEE_HANDLE_NULL;
		goto err;
	}

	/* Free potential previous transient object */
	if (sess->key_handle != TEE_HANDLE_NULL)
		TEE_FreeTransientObject(sess->key_handle);

	/* Allocate transient object according to target key size */
	res = TEE_AllocateTransientObject(TEE_TYPE_AES,
					  sess->key_size * 8,
					  &sess->key_handle);
	if (res != TEE_SUCCESS) {
		EMSG("Failed to allocate transient object");
		sess->key_handle = TEE_HANDLE_NULL;
		goto err;
	}

	key = TEE_Malloc(sess->key_size, 0);
	if (!key) {
		res = TEE_ERROR_OUT_OF_MEMORY;
		goto err;
	}

	TEE_InitRefAttribute(&attr, TEE_ATTR_SECRET_VALUE, key, sess->key_size);

	res = TEE_PopulateTransientObject(sess->key_handle, &attr, 1);
	if (res != TEE_SUCCESS) {
		EMSG("TEE_PopulateTransientObject failed, %x", res);
		goto err;
	}

	res = TEE_SetOperationKey(sess->op_handle, sess->key_handle);
	if (res != TEE_SUCCESS) {
		EMSG("TEE_SetOperationKey failed %x", res);
		goto err;
	}

	return res;

err:
	if (sess->op_handle != TEE_HANDLE_NULL)
		TEE_FreeOperation(sess->op_handle);
	sess->op_handle = TEE_HANDLE_NULL;

	if (sess->key_handle != TEE_HANDLE_NULL)
		TEE_FreeTransientObject(sess->key_handle);
	sess->key_handle = TEE_HANDLE_NULL;

	return res;
}

static TEE_Result set_aes_key(void *session, char *key, uint32_t key_sz){

	struct aes_cipher *sess;
	TEE_Attribute attr;
	TEE_Result res;

	/* Get ciphering context from session ID */
	sess = (struct aes_cipher *)session;

	//---------------------------------------------------------------
	if (key_sz != sess->key_size) {
		EMSG("Wrong key size %" PRIu32 ", expect %" PRIu32 " bytes",
		     key_sz, sess->key_size);
		return TEE_ERROR_BAD_PARAMETERS;
	}

	TEE_InitRefAttribute(&attr, TEE_ATTR_SECRET_VALUE, key, key_sz);

	TEE_ResetTransientObject(sess->key_handle);
	res = TEE_PopulateTransientObject(sess->key_handle, &attr, 1);
	if (res != TEE_SUCCESS) {
		EMSG("TEE_PopulateTransientObject failed, %x", res);
		return res;
	}

	TEE_ResetOperation(sess->op_handle);
	res = TEE_SetOperationKey(sess->op_handle, sess->key_handle);
	if (res != TEE_SUCCESS) {
		EMSG("TEE_SetOperationKey failed %x", res);
		return res;
	}

	return res;
}

static TEE_Result reset_aes_iv(void *session, char *aad, size_t aad_sz,
                     char *nonce, size_t nonce_sz, size_t payload_sz){

	struct aes_cipher *sess;

	/* Get ciphering context from session ID */
	sess = (struct aes_cipher *)session;

	TEE_AEInit(sess->op_handle, nonce, nonce_sz, 16*8/* tag_len in bits */, aad_sz /*aad_len*/,
						payload_sz /*plaintext_len*/);

	TEE_AEUpdateAAD(sess->op_handle, aad, aad_sz);

	return TEE_SUCCESS;
}

static TEE_Result set_key(void *session, uint32_t param_types,
				TEE_Param params[4])
{
	TEE_Result res = TEE_ERROR_OUT_OF_MEMORY;
	const uint32_t exp_param_types = TEE_PARAM_TYPES(TEE_PARAM_TYPE_MEMREF_INPUT,
				TEE_PARAM_TYPE_MEMREF_INPUT,
				TEE_PARAM_TYPE_MEMREF_INPUT,
				TEE_PARAM_TYPE_NONE);
	struct aes_cipher *sess;
    Connection connection;

	sess = (struct aes_cipher *)session;
    char nonce[12] = { 0 };
    size_t nonce_sz = 12;

    alloc_resources(sess, TA_AES_ALGO_GCM, 16, TA_AES_MODE_DECODE);
    set_aes_key(sess, module_key, 16);
    reset_aes_iv(sess, params[0].memref.buffer, params[0].memref.size, nonce, nonce_sz, params[1].memref.size);

    char *tag;
    tag = params[0].memref.buffer;
    char *temp;

    void *decrypted_key = NULL;
    void *tag_void = NULL;

   //==========================================
    decrypted_key = TEE_Malloc(16, 0);
    tag_void = TEE_Malloc(params[2].memref.size, 0);
	if (!decrypted_key || !tag_void)
		goto out;

	TEE_MemMove(tag_void, params[2].memref.buffer, params[2].memref.size);

	res = TEE_AEDecryptFinal(sess->op_handle, params[1].memref.buffer,
				 params[1].memref.size, decrypted_key, &params[2].memref.size, tag_void,
				 params[2].memref.size);

	if (!res) {
      temp = decrypted_key;
      for (int j = 0; j < 16; j++){
		  connection.connection_key[j]= temp[j];
	  }

	  connection.nonce = 0;
	  connection.encryption = tag[0] & 0xFF;

	  int j = 0;
      connection.conn_id = 0;
      for(int n=2; n>=1; --n){
         connection.conn_id = connection.conn_id + (( tag[n] & 0xFF ) << (8*j));
         ++j;
      }
      j = 0;
      connection.io_id = 0;
      for(int n=4; n>=3; --n){
         connection.io_id = connection.io_id + (( tag[n] & 0xFF ) << (8*j));
         ++j;
      }
	  total_node = total_node + 1;
      connections_add(&connection);
    }

out:
	TEE_Free(decrypted_key);
    TEE_Free(tag_void);

	return res;
}

//======================================================================

static TEE_Result attest(void *session, uint32_t param_types,
				TEE_Param params[4])
{
	TEE_Result res = TEE_ERROR_OUT_OF_MEMORY;
	const uint32_t exp_param_types = TEE_PARAM_TYPES(TEE_PARAM_TYPE_MEMREF_INPUT,
				TEE_PARAM_TYPE_MEMREF_OUTPUT,
				TEE_PARAM_TYPE_NONE,
				TEE_PARAM_TYPE_NONE);
	struct aes_cipher *sess;

	sess = (struct aes_cipher *)session;

	// ------------ Call PTA ---------**************************************************
	TEE_TASessionHandle pta_session = TEE_HANDLE_NULL;
	uint32_t ret_origin = 0;
	uint32_t pta_param_types = TEE_PARAM_TYPES( TEE_PARAM_TYPE_MEMREF_OUTPUT,
											TEE_PARAM_TYPE_NONE, TEE_PARAM_TYPE_NONE,
											TEE_PARAM_TYPE_NONE);

	TEE_Param pta_params[TEE_NUM_PARAMS];

	// prepare the parameters for the pta
	pta_params[0].memref.buffer = module_key;
	pta_params[0].memref.size = 16;

	// ------------ Open Session to PTA ---------
	res = TEE_OpenTASession(&pta_attestation_uuid, 0, 0, NULL, &pta_session,
				&ret_origin);
	if (res != TEE_SUCCESS)
		return res;

	// ------------ Invoke command at PTA (get_module key) ---------
	res = TEE_InvokeTACommand(pta_session, 0, ATTESTATION_CMD_GET_MODULE_KEY,
								pta_param_types, pta_params, &ret_origin);
	if (res != TEE_SUCCESS)
		return res;

	// ------------ Close Session to PTA ---------
	TEE_CloseTASession(pta_session);

	//*******************************************************************************
    char nonce[12] = { 0 };
    size_t nonce_sz = 12;
	// challenge =  param[0] --> aad
    alloc_resources(sess, TA_AES_ALGO_GCM, 16, TA_AES_MODE_ENCODE);
    set_aes_key(sess, module_key, 16);
    reset_aes_iv(sess, params[0].memref.buffer, params[0].memref.size, nonce, nonce_sz, 0);

	unsigned char challenge[16]={0};
	memcpy(challenge, params[0].memref.buffer, 16);
    DMSG("Module key: %x %x", module_key[0], module_key[1]);
    DMSG("Challenge: %x %x", challenge[0], challenge[1]);

    void *tag = TEE_Malloc(16, 0);
    uint32_t len = 0, tag_len = 16;

	res = TEE_AEEncryptFinal(sess->op_handle, NULL,
				 0, NULL, &len, tag, &tag_len);

	if (!res) {
		params[1].memref.size = 16;
		TEE_MemMove(params[1].memref.buffer, tag, params[1].memref.size);
    }
    else {
    	DMSG("MAC generation failed: %d", res);
    }

	TEE_Free(tag);

	return res;

}

//======================================================================
void handle_output(void *session, uint8_t *num, uint16_t output_id, unsigned char *conn_id,
					 unsigned char *data, uint32_t data_len, unsigned char *tag) {

	struct aes_cipher *sess;
	sess = (struct aes_cipher *)session;

	const void *text = data; // for AES

	BitSequence output[data_len];
	BitSequence tag_spongent[16];
	BitSequence data_spongent[data_len];
	memcpy(data_spongent, data, data_len); // for spongent

	uint8_t index = 0;

	int arr[total_node];
	find_connections(output_id, arr, num);

	for(int i = 0; i < *num; i++){

		Connection* connection = connections_get(arr[i]);
		char nonce[12] = { 0 };
    	size_t nonce_sz = 12;

		unsigned char aad[2] = { 0 };
		int j = 1;
    	for(int m = 0; m < 2; m++){
    		aad[m] = ((connection->nonce) >> (8*j)) & 0xFF; // ########
    		j--;
    	}

		unsigned char conn_id_array[2] = { 0 };
		int c = 1;
    	for(int m = 0; m < 2; m++){
    		conn_id_array[m] = ((connection->conn_id) >> (8*c)) & 0xFF;
    		c--;
    	}

		memcpy(conn_id + (2 * i), conn_id_array, 2);

		//*************** ^ ^ *******************************************************

		if(connection->encryption == AES){

    		alloc_resources(sess, TA_AES_ALGO_GCM, 16, TA_AES_MODE_ENCODE);
    		set_aes_key(sess, connection->connection_key, 16); //#######
    		reset_aes_iv(sess, aad, 2, nonce, nonce_sz, data_len);

			void *encrypt = NULL;
			void *tag_void = NULL;
			uint32_t sz = 16;

			encrypt = TEE_Malloc(data_len, 0);
			tag_void = TEE_Malloc(16, 0);

			TEE_Result res = TEE_AEEncryptFinal(sess->op_handle, text, data_len,
					encrypt, &data_len, tag_void, &sz);

			if (!res) {

				data[index] = data_len & 0xFF;
				memcpy(data + index + 1, encrypt, data_len);
				memcpy(tag + (16 * i), tag_void, 16);
				TEE_Free(encrypt);
    			TEE_Free(tag_void);
			}
		} // if AES
		else {

			SpongentWrap(connection->connection_key, aad, 16, data_spongent, data_len * 8, output, tag_spongent, 0);

			data[index] = data_len & 0xFF;
			memcpy(data + index + 1, output, data_len);
			memcpy(tag + (16 * i), tag_spongent, 16);
		} // if SPONGENT

		index = index + data_len + 1;
		connection->nonce = connection->nonce + 1; //######

    } // for

}
//=========================================================================
TEE_Result handle_input(void *session, uint32_t param_types, TEE_Param params[4]){

	const uint32_t exp_param_types = TEE_PARAM_TYPES(TEE_PARAM_TYPE_VALUE_INOUT,
				TEE_PARAM_TYPE_MEMREF_OUTPUT,
				TEE_PARAM_TYPE_MEMREF_INOUT,
				TEE_PARAM_TYPE_MEMREF_INOUT);

	TEE_Result res;
	struct aes_cipher *sess;
	sess = (struct aes_cipher *)session;
	uint32_t size = params[0].value.a;
	unsigned char* data;
	data = malloc(size);

	Connection* connection = connections_get(params[0].value.b);

	char nonce[12] = { 0 };
    size_t nonce_sz = 12;

	unsigned char aad[2] = { 0 };
	int j = 1;
    for(int m = 0; m < 2; m++){
    	aad[m] = ((connection->nonce) >> (8*j)) & 0xFF; // ########
    	j--;
    }

	//---------------------------------------------------------------
	if(connection->encryption == AES){

		alloc_resources(sess, TA_AES_ALGO_GCM, 16, TA_AES_MODE_DECODE);
    	set_aes_key(sess, connection->connection_key, 16); //#######
    	reset_aes_iv(sess, aad, 2, nonce, nonce_sz, size);

    	void *decrypted_data = NULL;
    	void *tag_void = NULL;

    	decrypted_data = TEE_Malloc(size, 0);
    	tag_void = TEE_Malloc(16, 0);

		TEE_MemMove(tag_void, params[3].memref.buffer, 16);

		res = TEE_AEDecryptFinal(sess->op_handle, params[2].memref.buffer, size,
						decrypted_data, &size, tag_void, 16);

		if (!res) {
      		memcpy(data, decrypted_data, size);
	  		for(int i = 0; i < size; i++){
		  		printf("%2X", data[i]);
	  		}
			printf("\n");
	  		TEE_Free(decrypted_data);
	  		TEE_Free(tag_void);
		}
	}// if AES
	else{

		BitSequence tag_spongent[16]; 	// TAG length is the same as the key length. 16 bytes.
		BitSequence encrypted_data[size];

		for (int n = 0; n < size; n++){
			encrypted_data[n] = ((uint8_t *)params[2].memref.buffer)[n];
		}

		for (int n = 0; n < 16; n++){
			tag_spongent[n] = ((uint8_t *)params[3].memref.buffer)[n];
		}

		SpongentUnwrap(connection->connection_key, aad, 16, encrypted_data,
										size * 8, data, tag_spongent);

		for(int i = 0; i < size; i++){
			printf("%2X", data[i]);
	  	}
		printf("\n");
	}// if spongent

	connection->nonce = connection->nonce + 1;

	find_input_func(session, param_types, params, connection->io_id, data, size);

	return TEE_SUCCESS;
}

// Called when the TA is created =======================================
TEE_Result TA_CreateEntryPoint(void) {
   DMSG("=============== TA_CreateEntryPoint ================");
   return TEE_SUCCESS;
}

// Called when the TA is destroyed
void TA_DestroyEntryPoint(void) {
   DMSG("=============== TA_DestroyEntryPoint ===============");
}

// open session
TEE_Result TA_OpenSessionEntryPoint(uint32_t __unused param_types,
					TEE_Param __unused params[4],
					void __unused **session)
{
   DMSG("=========== TA_OpenSessionEntryPoint ===============");

	struct aes_cipher *sess;
	sess = TEE_Malloc(sizeof(*sess), 0);
	if (!sess)
		return TEE_ERROR_OUT_OF_MEMORY;

	sess->key_handle = TEE_HANDLE_NULL;
	sess->op_handle = TEE_HANDLE_NULL;

	*session = (void *)sess;

	return TEE_SUCCESS;
}

// close session
void TA_CloseSessionEntryPoint(void *session)
{
   DMSG("========== TA_CloseSessionEntryPoint ===============");

	struct aes_cipher *sess;

	/* Get ciphering context from session ID */
	sess = (struct aes_cipher *)session;

	/* Release the session resources */
	if (sess->key_handle != TEE_HANDLE_NULL)
		TEE_FreeTransientObject(sess->key_handle);
	if (sess->op_handle != TEE_HANDLE_NULL)
		TEE_FreeOperation(sess->op_handle);
	TEE_Free(sess);
}

// invoke command
TEE_Result TA_InvokeCommandEntryPoint(void *session, uint32_t cmd, uint32_t param_types,
					TEE_Param params[4])
{
	switch (cmd) {
	case SET_KEY:
		return set_key(session, param_types, params);
	case ATTEST:
		return attest(session, param_types, params);
	case HANDLE_INPUT:
		return handle_input(session, param_types, params);
	case ENTRY:
		return entry(session, param_types, params);
	default:
		EMSG("Command ID 0x%x is not supported", cmd);
		return TEE_ERROR_NOT_SUPPORTED;
	}
}
