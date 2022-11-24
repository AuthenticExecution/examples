#!/bin/bash

set -e

# Wait untill all EMs are up and running
sleep 10

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

echo "Setup complete"
sleep 360000000000