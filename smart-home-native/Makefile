WEB_URL     ?= https://node-sgx:48879
API         ?=
PORT        ?= 48879
TOKEN       ?= secret-token

PAYLOAD      = $(shell printf "%x" $(PORT))$(shell echo -n $(TOKEN) | xxd -p)

test:

init:
	./init-webserver.sh $(PAYLOAD)

get:
	curl --cacert cert.pem -i $(WEB_URL)/$(API) -H "Authorization: Bearer $(TOKEN)"

shell:
	docker exec -it smart-home-native-admin-1 bash