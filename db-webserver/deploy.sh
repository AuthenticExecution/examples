#!/bin/bash

set -e

echo "Waiting until all the EMs are ready.."
sleep 10

# deploy
echo "Deploying modules.."
reactive-tools --verbose deploy descriptor.json --result res.json
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --verbose attest res.json
sleep 2

# connect
echo "Establishing connections.."
reactive-tools --verbose connect res.json

echo "Setup complete"
sleep 3600
