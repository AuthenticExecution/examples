KB			?= 0
_DATA_SIZE	?= $(shell python -c "print('{:04x}'.format($(KB)))")
DESCRIPTOR	?= descriptor_sensing.json

start:
	reactive-tools call res.json --module sensor --entry start --arg $(_DATA_SIZE)

run: clean
	cp $(DESCRIPTOR) descriptor.json
	docker compose up --remove-orphans --force-recreate

run_simulation: clean
	screen -A -m -d -S v2x-sims ./run_sim.sh  &

shell:
	docker exec -it supply-chain-admin-1 bash

clean:
	docker compose down