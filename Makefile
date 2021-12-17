AM_IMAGE         ?= authexec/attestation-manager:sgx
AM_REPO          ?= https://github.com/AuthenticExecution/attestation-manager.git
IAS_ROOT_CA      ?= https://certificates.trustedservices.intel.com/Intel_SGX_Attestation_RootCA.pem

all: get_am_signature get_am_keys generate_keys get_root_ca

get_am_signature:
	mkdir -p cred
	docker run --rm -it --detach --name tmp_container $(AM_IMAGE) bash
	docker cp tmp_container:/home/enclave/enclave.sig cred/manager.sig
	docker stop tmp_container

get_am_keys:
	mkdir -p cred
	git clone --depth 1 $(AM_REPO) __tmp__
	cp __tmp__/keys/sp_privkey.pem cred/manager_privkey.pem
	cp __tmp__/keys/sp_pubkey.pem cred/manager_pubkey.pem
	rm -rf __tmp__

generate_keys:
	mkdir -p cred
	openssl genrsa -3 3072 > cred/vendor_key.pem
	openssl genrsa -f4 -out cred/sp_privkey.pem 2048
	openssl rsa -in cred/sp_privkey.pem -outform PEM -pubout -out cred/sp_pubkey.pem

get_root_ca:
	mkdir -p cred
	curl $(IAS_ROOT_CA) > cred/ias_root_ca.pem

clean:
	rm -rf __tmp__
	find . -type 'd' -name cred -exec rm -rv {} +
	find . -type 'd' -name build -exec rm -rv {} +