#!/bin/bash

set -e

# TODO: find a better way
echo "Waiting until all the EMs are ready.."
sleep 2

# deploy
echo "Deploying modules.."
reactive-tools --verbose deploy descriptor.json --result res.json

# TODO: is it really necessary?
echo "Waiting until all the modules are up and running.."
sleep 2

# attest
echo "Attesting modules.."
reactive-tools --verbose attest res.json

# connect
echo "Establishing connections.."
reactive-tools --verbose connect res.json

echo "Setup complete"
sleep 3600
