#!/bin/bash

set -e

echo "Initializing web server with payload $1"
reactive-tools request res.json --connection init-server --arg $1 --out cert.der > /dev/null 2>&1

echo "Converting certificate from DER to PEM"
openssl x509 -inform der -in cert.der -out cert.pem > /dev/null 2>&1

echo "All done! You can query the server with 'curl --cacert cert.pem <url>'"
