KB			?= 0
_DATA_SIZE	?= $(shell python -c "print('{:04x}'.format($(KB)))")

start_shipment:
	reactive-tools call res.json --module sensor --entry trigger_start_shipment --arg 0000

end_shipment:
	reactive-tools call res.json --module sensor --entry trigger_end_shipment --arg 0000

sense:
	reactive-tools call res.json --module sensor --entry trigger_start_sensing --arg $(_DATA_SIZE)

request:
	@curl node-sgx

run: clean
	docker compose up --remove-orphans --force-recreate

shell:
	docker exec -it supply-chain-admin-1 bash

clean:
	docker compose down