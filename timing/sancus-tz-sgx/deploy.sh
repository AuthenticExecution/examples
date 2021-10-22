#!/bin/bash

set -e

# Attesting the Attestation Manager
echo "Attesting AM.."
python3 run_attester.py manager.yaml

sleep 1

# Initialize the Attestation Manager for the SGX attestation
echo "Initializing AM.."
attman-cli --config manager.yaml --request init-sgx --data init_sgx.yaml

# deploy
echo "Deploying modules.."
reactive-tools --manager deploy descriptor.json --result res.json

# TODO: is it really necessary?
echo "Waiting until all the modules are up and running.."
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --manager attest res.json

# connect
echo "Establishing connections.."
reactive-tools --manager connect res.json

echo "Setup complete"
sleep 5

echo "STARTING PING-PONG 8 BYTES"
for i in {1..10}
do
   echo "STARTING NEW ITERATION: $i"
   reactive-tools call res.json --module ping --entry start --arg 0800
   sleep 5
done

echo "STARTING PING-PONG 32 BYTES"
for i in {1..10}
do
   echo "STARTING NEW ITERATION: $i"
   reactive-tools call res.json --module ping --entry start --arg 2000
   sleep 5
done
