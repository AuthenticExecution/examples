# db-webserver

This is a simple example with two SGX/native modules called `db` and
`webserver`. The behavior of this example is very basic:

- `webserver` runs an HTTP server whose webpage shows the total number of
  requests received so far.
- `db` stores the number of requests on a global variable, and provides it to
  `webserver` upon request.

Two connections are needed:

- An output-input connection from `webserver` to `db` when a new HTTP GET
  request is received, which notifies `db` that the number needs to be
  incremented.
- A request-handler connection from `webserver` to `db`, used to retrieve the
  total number of requests when a HTTP get occurs (in order to attach it to the
  HTTP response).

## Try it out

The example here runs an SGX application, i.e., `db` and `webserver` run as
enclave. However, if you want to try it out on a linux machine without SGX, do
the following steps:

- In `descriptor.json`, replace the type of nodes and modules
from `sgx` to `native`;
- In `docker-compose.yml`, remove any devices and volumes from `node-sgx` and
  set `EM_SGX` to `false`.

```bash
# deploy the application in detached mode
docker-compose up -d

# wait until the setup is complete
# you can run `docker logs -f db-webserver_admin_1` and wait for a "Setup complete" log

# open a shell on the `admin` console
docker exec -it db-webserver_admin_1 bash

# initialize the web server: we will call the `init` entry point of `webserver`
reactive-tools call res.json --module webserver --entry init

# send requests to the web server
# repeat this multiple times, noticing that the returned number increases each time
curl node-sgx

# exit the shell using CTRL-D

# close the application
docker-compose down
```