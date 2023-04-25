KB			   ?= 0
_DATA_SIZE	   ?= $(shell python -c "print('{:04x}'.format($(KB)))")
DESCRIPTOR	   ?= descriptor_sensing.json
RESULT         ?= data.csv
OUT_PLOT_FLD   ?= results
SIM_MAX_SIZE   ?= 10
SIM_ITERATIONS ?= 100
SIM_IS_SENSOR  ?= 1

start:
	reactive-tools call res.json --module sensor --entry start --arg $(_DATA_SIZE)

run: clean
	cp $(DESCRIPTOR) descriptor.json
	docker compose up --remove-orphans --force-recreate

run_simulation: clean
	screen -L -Logfile out.log -A -m -d -S supply-chain-sim scripts/run_sim.sh scripts/run_collect.py $(RESULT) $(SIM_MAX_SIZE) $(SIM_ITERATIONS) $(SIM_IS_SENSOR) &
	@sleep 0.5
	screen -r supply-chain-sim -X colon "logfile flush 0.1^M"

run_sensor: clean
	screen -L -Logfile out.log -A -m -d -S supply-chain-sim scripts/run_sim.sh scripts/run_collect.py $(RESULT) $(SIM_MAX_SIZE) $(SIM_ITERATIONS) 1 &
	@sleep 0.5
	screen -r supply-chain-sim -X colon "logfile flush 0.1^M"

run_shipment: clean
	screen -L -Logfile out.log -A -m -d -S supply-chain-sim scripts/run_sim.sh scripts/run_collect.py $(RESULT) 1 $(SIM_ITERATIONS) 0 &
	@sleep 0.5
	screen -r supply-chain-sim -X colon "logfile flush 0.1^M"

shell:
	docker exec -it supply-chain-admin-1 bash

clean:
	screen -X -S supply-chain-sim quit || true
	docker compose down
	rm -rf descriptor.json out.csv out.json out.log res.json tee-evaluation.zip tee.png tee.tex data.csv

plot:
	python scripts/plot.py $(RESULT) $(OUT_PLOT_FLD)

shipment:
	python scripts/shipment.py $(RESULT)

logs:
	docker compose logs -f

zip:
	zip -r tee-evaluation.zip receiver_1 receiver_2 sensor_1.c sensor_2.c .env scripts results deploy.sh descriptor_sensing.json descriptor_shipment.json docker-compose.yml Makefile README.md