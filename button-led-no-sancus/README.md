# button-led-no-sancus

This example is analogous to [button-led](../button-led), except that Sancus modules have been replaced with simple native ones.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and [docker-compose](https://docs.docker.com/compose/install/) to run this example.
- SGX capabilities and software dependencies (SGX driver, AESM service)
- [TZ dependencies](https://github.com/AuthenticExecution/event-manager-trustzone)

## Run the example

```bash
# Deploy the system
docker-compose up

# Wait until the "admin" container prints "Setup complete"
# The following commands should be launched from another terminal

# Open a shell in the "admin" container
docker exec -it button-led-no-sancus_admin_1 bash

# Initialize the web server and retrieve the self-signed certificate
# You can actually use a different port than "beef" if you want
reactive-tools request res.json --connection init-server --arg beef --out cert.der

# Convert the retrieved certificate in PEM format
openssl x509 -inform der -in cert.der -out cert.pem

# Try querying the web server to get the number of button presses (check the port)
curl --cacert cert.pem https://node-sgx:48879 # returns 0

# Simulate a button press
reactive-tools --verbose output res.json --connection trigger-btn

# Query the web server again
curl --cacert cert.pem https://node-sgx:48879 # returns 1

### Cleanup ###
# Go back on the first terminal, press CTRL-C to stop the containers

# Delete the network and containers
docker-compose down
```
