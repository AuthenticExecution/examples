WEB_URL     ?= https://node-sgx:48879
API         ?=

init:
	./init-webserver.sh

get:
	curl --cacert cert.pem -i $(WEB_URL)/$(API)

shell:
	docker exec -it smart-home-native-admin-1 bash