#!/bin/bash

set -e

# TODO: find a better way
echo "Waiting until all the EMs are ready.."
sleep 30

# deploy
echo "Deploying modules.."
reactive-tools --debug --manager deploy descriptor.json --result res.json
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --debug --manager attest res.json
sleep 2

# connect
echo "Establishing connections.."
reactive-tools --debug --manager connect res.json

echo "Setup complete"
sleep 3600
