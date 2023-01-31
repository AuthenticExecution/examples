ADMIN_CONTAINER    ?= sgx-tz-admin-1

IN                 ?= logs
OUT                ?= log.json

collect:
	mkdir -p $(IN)
	docker compose up -d --force-recreate
	until docker logs $(ADMIN_CONTAINER) 2> /dev/null | grep 'ALL DONE' ; do sleep 5; done
	docker compose kill
	#docker cp sgx-tz-node-trustzone-1:/opt/optee/nw.log $(IN)
	docker cp sgx-tz-node-trustzone-1:/opt/optee/sw.log $(IN)
	docker logs sgx-tz-node-sgx-1 > $(IN)/sgx.log
	cat $(IN)/sw.log $(IN)/sgx.log > $(IN)/aggregated.log
	docker compose down 
	python3 collect.py $(IN)/aggregated.log $(IN)/$(OUT)
