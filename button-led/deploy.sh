#!/bin/bash

set -e

# TODO: find a better way
echo "Waiting until all the EMs are ready.."
sleep 30

# Attesting the Attestation Manager
echo "Attesting AM.."
python3 run_attester.py

# Wait until AM is ready
sleep 1

# Initialize the Attestation Manager for the SGX attestation
echo "Initializing AM.."
attman-cli --config manager.yaml --request init-sgx --data init_sgx.yaml

# deploy
echo "Deploying modules.."
reactive-tools --verbose --manager deploy descriptor.json --result res.json
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --verbose --manager attest res.json
sleep 2

# connect
echo "Establishing connections.."
reactive-tools --verbose --manager connect res.json

# init LED and attest pmodled
# note: we give a dummy argument because led_driver would otherwise fail
# due to a bug that makes sancus_unwrap fail if payload size is zero
echo "Initializing and attesting MMIO LED.."
reactive-tools --verbose output res.json --connection init-led --arg 0000

echo "Setup complete"
sleep 3600
