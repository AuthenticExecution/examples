#!/bin/bash

set -e

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

echo "STARTING PING-PONG"
# 8 bytes: 0800 | 32 bytes: 2000

for i in {1..10}
do
   echo "STARTING NEW ITERATION: $i"
   reactive-tools call res.json --module ping --entry start --arg 0800
   sleep 5
done

echo "ALL DONE"
