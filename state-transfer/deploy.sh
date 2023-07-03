#!/bin/bash

set -e

# Attesting the Attestation Manager
echo "Attesting AM.."
python3 run_attester.py manager.yaml

# Wait until AM is ready
sleep 1

# Initialize the Attestation Manager for the SGX attestation
echo "Initializing AM.."
attman-cli --config manager.yaml --request init-sgx --data init_sgx.yaml

# Wait until all EMs are up and running
sleep 30

# deploy
echo "Deploying modules.."
reactive-tools --manager deploy descriptor.json --result res.json
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --manager attest res.json
sleep 2

# connect
echo "Establishing connections.."
reactive-tools --manager connect res.json

echo "Setup complete"
sleep 5

echo "Curl before update"
reactive-tools call res.json --module counter --entry init
sleep 1
[ $(curl node-sgx-1) -eq 0 ]
reactive-tools --verbose request res.json --connection get-requests

echo "Updating source.."
python update.py res.json counter node_sgx_2
reactive-tools --manager --timing update res.json --module counter --entry __save --output __transfer --input __restore
sleep 5

echo "Curl after update"
reactive-tools call res.json --module counter --entry init
sleep 1
[ $(curl node-sgx-2) -eq 1 ]
reactive-tools --verbose request res.json --connection get-requests

echo "ALL DONE"
