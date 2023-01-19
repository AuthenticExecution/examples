# smart-home-native

## Prerequisites

Check the main [README](../README.md)

## TZ: QEMU vs i.MX6

- Make sure the hostnames in `descriptor.json` are correct
- In `docker-compose.yml`:
    - Set the `IMX` env. variable in `node-trustzone` accordingly
    - Check if all ports are correct
    - Ensure all the UART devices for Sancus are correct

## Run the example

```bash
docker compose up
```