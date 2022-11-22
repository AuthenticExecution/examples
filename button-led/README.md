# button-led

[Video](https://drive.google.com/file/d/17507pl12mycp1ELxH8cLghydNJlxVawi/view?usp=sharing)

## Prerequisites

Check the main [README](../README.md)

- [Simulated version](../button-led-native) (only Docker and docker-compose needed)

## Before running the example

### Check the key of `led_driver` in `descriptor.json`

The module key of `led_driver` might change every time the event manager is
re-built. Double check that the key is correct by following the instructions
[here](https://github.com/AuthenticExecution/event-manager-sancus).

### Sancus UART interfaces

Sometimes the UART interfaces might change order. Check `docker-compose.yml` to
ensure that:
- The device mounted to `/dev/RIOT` is the one typically for loading binaries
  into Sancus.
- The device mounted to `/dev/UART` is the other one of the same Sancus board,
  which will be used for sending and receiving data.

## Run the example

```bash
# Deploy the system
docker-compose up

# Wait until the "admin" container prints "Setup complete"
# The following commands should be launched from another terminal

# Open a shell in the "admin" container
docker exec -it button-led_admin_1 bash

# Initialize the web server and retrieve the self-signed certificate
# You can actually use a different port than "beef" if you want
reactive-tools request res.json --connection init-server --arg beef --out cert.der

# Convert the retrieved certificate in PEM format
openssl x509 -inform der -in cert.der -out cert.pem

# Try querying the web server to get the number of button presses (check the port)
curl --cacert cert.pem https://node-sgx:48879 # returns 0

# Simulate a button press (giving a dummy argument due to a Sancus bug)
reactive-tools --verbose output res.json --connection trigger-btn --arg 0000

# Query the web server again
curl --cacert cert.pem https://node-sgx:48879 # returns 1

### Cleanup ###
# Go back on the first terminal, press CTRL-C to stop the containers

# Delete the network and containers
docker-compose down
```
