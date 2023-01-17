#!/bin/bash

set -e

echo "Waiting until all the EMs are ready.."
sleep 10

# deploy
echo "Deploying modules.."
reactive-tools deploy descriptor.json --result res.json
sleep 2

# attest
echo "Attesting modules.."
reactive-tools attest res.json
sleep 2

# connect
echo "Establishing connections.."
reactive-tools connect res.json

echo "Initializing web server"
make init

echo "Setup complete"

while true
do
	sleep 1
    reactive-tools call res.json --module temp_sensor --entry read_from_sensor > /dev/null 2>&1
    reactive-tools call res.json --module light_switch --entry check_switch > /dev/null 2>&1
    sleep 0.1
    reactive-tools call res.json --module gateway --entry check_heater > /dev/null 2>&1
done