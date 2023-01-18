#!/bin/bash

set -e

echo "Waiting until all the EMs are ready.."
sleep 20

# deploy
echo "Deploying modules.."
reactive-tools --debug deploy descriptor.json --result res.json
sleep 2

# attest
echo "Attesting modules.."
reactive-tools attest res.json
sleep 2

# connect
echo "Establishing connections.."
reactive-tools --debug connect res.json --connect-in-order

echo "Initializing and attesting MMIO LED.."
reactive-tools --verbose output res.json --connection init-led --arg 0000

echo "Initializing web server"
make init

echo "Setup complete"

while true
do
	sleep 1
    reactive-tools call res.json --module temp_sensor --entry read_from_sensor > /dev/null 2>&1
    sleep 0.1
    reactive-tools call res.json --module gateway --entry check_heater > /dev/null 2>&1
done