# Example Authentic Execution deployments

Each folder contains a different example of Authentic Execution deployment. Each example uses different TEE technologies, some of them combine multiple TEEs together. Check the `docker-compose.yml` and `descriptor.json` files to find out the exact topology of a deployment.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and [docker-compose](https://docs.docker.com/compose/install/) are mandatory to run the examples.

- Each TEE technology needs different prerequisites. Some components/drivers may be required. [More info](https://github.com/AuthenticExecution/env#prerequisites)

## Example: fosdem-21-native

No particular prerequisites are needed (except Docker and docker-compose).

```bash
# go to the folder
cd fosdem-21-native

# run deployment
docker-compose up

# Wait until the deployment is complete (the deployer prints "Setup complete")

# Open a new terminal and execute a shell on the deployer's container
docker exec -it fosdem-21-native_deployer_1 bash

# interact with the system
reactive-tools --verbose output res.json --connection init-server --arg beef
reactive-tools --verbose output res.json --connection trigger-btn
curl em-native-1:48879 # 1

# On the first terminal, stop the containers with CTRL-C

# Clean up
docker-compose down
```
