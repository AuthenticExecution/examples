KB			?= 0
_DATA_SIZE	?= $(shell python -c "print('{:04x}'.format($(KB)))")
DESCRIPTOR	?= descriptor_sensing.json

start_shipment:
	reactive-tools call res.json --module sensor --entry trigger_start_shipment --arg 0000

end_shipment:
	reactive-tools call res.json --module sensor --entry trigger_end_shipment --arg 0000

sense:
	reactive-tools call res.json --module sensor --entry trigger_start_sensing --arg $(_DATA_SIZE)

run: clean
	cp $(DESCRIPTOR) descriptor.json
	docker compose up --remove-orphans --force-recreate

run_simulation: clean
	screen -A -m -d -S v2x-sims ./run_sim.sh  &

shell:
	docker exec -it supply-chain-admin-1 bash

clean:
	docker compose down