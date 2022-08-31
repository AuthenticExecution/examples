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

# Wait untill all EMs are up and running
sleep 10

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

echo "Ping pong pre-update"
reactive-tools call res.json --module source --entry start --arg 0800
sleep 5

echo "Updating source.."
python update.py res.json source node_sgx_2
reactive-tools --manager --timing update res.json --module source
sleep 5

echo "Updating gw.."
python update.py res.json gw node_trustzone_2
reactive-tools --manager --timing update res.json --module gw
sleep 5

echo "Updating sink.."
python update.py res.json sink node_native_2
reactive-tools --manager --timing update res.json --module sink
sleep 5

echo "Ping pong post-update"
reactive-tools call res.json --module source --entry start --arg 0800

echo "ALL DONE"
