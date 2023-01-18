WEB_URL     ?= https://node-sgx:48879
API         ?=
PORT        ?= 48879
TOKEN       ?= secret-token
TEMP        ?= 18.0

PAYLOAD      = $(shell printf "%x" $(PORT))$(shell echo -n $(TOKEN) | xxd -p)

test:

init:
	./init-webserver.sh $(PAYLOAD)

get:
	curl --cacert cert.pem -i $(WEB_URL)/$(API) -H "Authorization: Bearer $(TOKEN)"

enable_heating:
	curl --cacert cert.pem -i $(WEB_URL)/enable-heating -H "Authorization: Bearer $(TOKEN)" -X POST -d  '{"enable" : true}'

disable_heating:
	curl --cacert cert.pem -i $(WEB_URL)/enable-heating -H "Authorization: Bearer $(TOKEN)" -X POST -d  '{"enable" : false}'

set_temp:
	curl --cacert cert.pem -i $(WEB_URL)/set-desired-temp -H "Authorization: Bearer $(TOKEN)" -X POST -d  '{"temp" : $(TEMP)}'

enable_switch:
	curl --cacert cert.pem -i $(WEB_URL)/enable-switch -H "Authorization: Bearer $(TOKEN)" -X POST -d  '{"enable" : true}'

disable_switch:
	curl --cacert cert.pem -i $(WEB_URL)/enable-switch -H "Authorization: Bearer $(TOKEN)" -X POST -d  '{"enable" : false}'

shell:
	docker exec -it smart-home-admin-1 bash