PYTHON_SCRIPT      ?= ../collect_measurements.py
ADMIN_CONTAINER    ?= encryption-time_admin_1

IN                 ?= log.txt
OUT                ?= log.csv
NUM_MEASUREMENTS   ?= 8
APPEND_INDEX       ?= 0

collect:
	docker-compose up -d
	until docker logs $(ADMIN_CONTAINER) 2> /dev/null | grep 'ALL DONE' ; do sleep 5; done
	docker-compose kill
	docker-compose logs -t --no-color | sort -u -k 3 > log.txt
	docker-compose down 
	python $(PYTHON_SCRIPT)  $(IN) $(OUT) $(NUM_MEASUREMENTS) $(APPEND_INDEX)