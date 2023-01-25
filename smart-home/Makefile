WEB_URL          ?= https://node-sgx:48879
API              ?=
PORT             ?= 48879
TOKEN            ?= secret-token
TEMP             ?= 18.0

IN               ?= logs
OUT              ?= out.json

PAYLOAD           = $(shell printf "%x" $(PORT))$(shell echo -n $(TOKEN) | xxd -p)

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

collect:
	mkdir -p $(IN)
	#docker compose up -d --force-recreate
	#until docker logs $(ADMIN_CONTAINER) 2> /dev/null | grep 'ALL DONE' ; do sleep 5; done
	#docker compose kill
	docker cp smart-home-node-trustzone-1:/opt/optee/nw.log $(IN)
	docker cp smart-home-node-trustzone-1:/opt/optee/sw.log $(IN)
	docker logs smart-home-node-sgx-1 > $(IN)/sgx.log
	#docker compose down 
	python3 collect.py $(IN) $(OUT)