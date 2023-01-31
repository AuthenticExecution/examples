PYTHON_SCRIPT      ?= collect.py
ADMIN_CONTAINER    ?= native-native-admin-1

IN                 ?= log.txt
OUT                ?= log.json

collect:
	mkdir -p logs
	docker compose up -d --force-recreate
	until docker logs $(ADMIN_CONTAINER) 2> /dev/null | grep 'ALL DONE' ; do sleep 5; done
	docker compose kill
	docker compose logs -t --no-color | sort -u -k 3 > logs/$(IN)
	docker compose down 
	python3 $(PYTHON_SCRIPT) logs/$(IN) logs/$(OUT)