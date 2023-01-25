#!/bin/bash

set -e

sleep 5

# deploy
echo "Deploying modules.."
reactive-tools deploy descriptor.json --result res.json

# TODO: is it really necessary?
echo "Waiting until all the modules are up and running.."
sleep 2

# attest
echo "Attesting modules.."
reactive-tools attest res.json

# connect
echo "Establishing connections.."
reactive-tools connect res.json

echo "Setup complete"
sleep 5

echo "STARTING PING-PONG"
# 8B: 0800 | 64B: 4000 | 512B: 0002 | 4kB: 0010 | 32kB: 0080

for i in {1..110}
do
   echo "STARTING NEW ITERATION: $i"
   reactive-tools call res.json --module ping --entry start --arg 0010 
   sleep 2
done

echo "ALL DONE"
