#!/bin/bash

set -e

# TODO: find a better way
echo "Waiting until all the EMs are ready.."
sleep 3

# deploy
echo "Deploying modules.."
reactive-tools --debug deploy descriptor.json --result res.json
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --verbose attest res.json
sleep 2

# connect
echo "Establishing connections.."
reactive-tools --verbose connect res.json

echo "Initializing web server"
make init

echo "Setup complete"

sleep 3600
while true
do
	sleep 1
    reactive-tools call res.json --module temp_sensor --entry read_from_sensor
    reactive-tools call res.json --module thermostat --entry check_heater
done