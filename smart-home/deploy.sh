#!/bin/bash

set -e

MODE=$1

echo "Waiting until all the EMs are ready.."
sleep 30

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
reactive-tools connect res.json --connect-in-order

echo "Initializing and attesting MMIO LEDs.."
reactive-tools output res.json --connection init-led-2 --arg 0000
reactive-tools output res.json --connection init-led-3 --arg 0000

echo "Initializing web server"
make init

echo "Setup complete"

if [ $MODE = "benchmark" ]; then
    sleep 5
    for i in {1..110}
    do
        echo "STARTING NEW ITERATION: $i"
        make enable_switch
        sleep 2
    done
elif [ $MODE = "update" ]; then
    sleep 5

    echo "Updating web.."
    reactive-tools --timing update res.json --module web
    sleep 5

    echo "Updating gateway.."
    reactive-tools --timing update res.json --module gateway
    sleep 5

    echo "Updating sensor.."
    python switch_node.py res.json sensor node_sancus_4
    reactive-tools --timing update res.json --module sensor 
    sleep 5
elif [ $MODE = "auto" ]; then
    while true
    do
        sleep 1
        reactive-tools call res.json --module sensor --entry read_from_sensor > /dev/null 2>&1
        sleep 0.1
        reactive-tools call res.json --module gateway --entry check_heater > /dev/null 2>&1
    done
else
    echo "sleeping"
    sleep 3600
fi

echo "ALL DONE"