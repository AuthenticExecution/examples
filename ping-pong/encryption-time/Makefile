PYTHON_SCRIPT      ?= collect.py
ADMIN_CONTAINER    ?= encryption-time-admin-1

IN                 ?= log.txt
OUT                ?= log.json

collect:
	docker compose up -d
	until docker logs $(ADMIN_CONTAINER) 2> /dev/null | grep 'ALL DONE' ; do sleep 5; done
	docker compose kill
	docker compose logs -t --no-color | sort -u -k 3 > log.txt
	docker compose down 
	python $(PYTHON_SCRIPT) $(IN) $(OUT)