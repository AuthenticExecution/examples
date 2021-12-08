#!/bin/bash

set -e

# TODO: find a better way
echo "Waiting until all the EMs are ready.."
sleep 10

# Attesting the Attestation Manager
echo "Attesting AM.."
python3 run_attester.py manager.yaml

# Initialize the Attestation Manager for the SGX attestation
echo "Initializing AM.."
attman-cli --config manager.yaml --request init-sgx --data init_sgx.yaml

# deploy
echo "Deploying modules.."
reactive-tools --verbose --manager deploy descriptor.json --result res.json

# TODO: is it really necessary?
echo "Waiting until all the modules are up and running.."
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --verbose --manager attest res.json

# connect
echo "Establishing connections.."
reactive-tools --verbose --manager connect res.json

echo "Setup complete"
sleep 3600
