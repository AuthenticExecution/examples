#!/bin/bash

set -e

COLLECT_DATA=$1
DATA_SIZE=$2
ITERATIONS=$3

# Wait until all EMs are up and running
sleep 30

# deploy
echo "Deploying modules.."
reactive-tools --verbose deploy descriptor.json --result res.json
sleep 5

# attest
echo "Attesting modules.."
reactive-tools --verbose attest res.json
sleep 5

# connect
echo "Establishing connections.."
reactive-tools --verbose connect res.json
sleep 5

echo "Initializing receiver.."
reactive-tools call res.json --module receiver --entry init

echo "Setup complete"

if [ $COLLECT_DATA -eq 1 ]; then
    echo "STARTING DATA SENSING"
    for (( i=1; i<=$ITERATIONS; i++ )); do
        echo "STARTING NEW ITERATION: $i"
        sleep_time=`expr $DATA_SIZE + 1`
        make start KB=$DATA_SIZE
        sleep $sleep_time
    done
else
    echo "SLEEPING"
    sleep 360000000000
fi