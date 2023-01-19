WEB_URL          ?= https://node-sgx:48879
API              ?=
PORT             ?= 48879
TOKEN            ?= secret-token
TEMP             ?= 18.0

PAYLOAD           = $(shell printf "%x" $(PORT))$(shell echo -n $(TOKEN) | xxd -p)

SANCUS_IMAGE      = authexec/event-manager-sancus:latest
SGX_IMAGE         = authexec/event-manager-sgx:latest
AESM_CLIENT_IMAGE = authexec/aesm-client:latest

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

run_sgx:
	docker run --rm --detach --network=host -v /var/run/aesmd/:/var/run/aesmd --name aesm-client $(AESM_CLIENT_IMAGE) >/dev/null 2>&1 || true
	docker run -d --rm --network=host --device /dev/isgx -v /var/run/aesmd/:/var/run/aesmd/ -e EM_PORT=5000 -e EM_LOG=info -e EM_THREADS=16 -e EM_PERIODIC_TASKS=false -e EM_SGX=true --name event-manager-5000 $(SGX_IMAGE)

run_sancus:
	docker run -d -p 6000:6000 -e PORT=6000 -e ELF=reactive.elf --device=/dev/ttyUSB2:/dev/RIOT --device=/dev/ttyUSB3:/dev/UART --rm --name event-manager-6000 $(SANCUS_IMAGE)
	docker run -d -p 7000:7000 -e PORT=7000 -e ELF=reactive_led.elf --device=/dev/ttyUSB0:/dev/RIOT --device=/dev/ttyUSB1:/dev/UART --rm --name event-manager-7000 $(SANCUS_IMAGE)
	docker run -d -p 8000:8000 -e PORT=8000 -e ELF=reactive_led.elf --device=/dev/ttyUSB4:/dev/RIOT --device=/dev/ttyUSB5:/dev/UART --rm --name event-manager-8000 $(SANCUS_IMAGE)

run_trustzone:
	screen -L -Logfile nw.txt -dmS tz-nw /dev/ttyUSB1 115200
	screen -L -Logfile sw.txt -dmS tz-sw /dev/ttyUSB0 115200

clean:
	@docker stop aesm-client || true
	@docker stop event-manager-5000 || true
	@docker stop event-manager-6000 || true
	@docker stop event-manager-7000 || true
	@docker stop event-manager-8000 || true