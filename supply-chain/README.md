# Supply chain example

## Preliminaries

```bash
# fetch the latest docker images
make -C .. update_images

# generate credentials
make -C .. credentials
cp -r ../cred .

# (SGX only) copy settings file and add own SPID and subscription keys
cp ../settings_template.json cred/settings.json
nano cred/settings.json
```

## Run

```bash
# Run application
# <descriptor_file>: {descriptor_sensing,descriptor_shipment}.json
make run DESCRIPTOR=<descriptor_file>

# (once finished) close application
CTRL-C
```

## Interact with the application

```bash
# (in a new shell) open the admin console
# note: only after deployment is complete ("Setup complete" is printed out)
make shell

# (descriptor_sensing.json only) send sensor data
# <num_kb>: size of sensor data in kilobytes
make sense KB=<num_kb>

# (descriptor_shipment.json only) start shipment
make start_shipment

# (descriptor_shipment.json only) end shipment
make end_shipment
```

## Collect data

```bash
# <out_file>: desired output file name
# <max_size>: maximum size of data in KBs
# <num_iterations>: num of iterations for each run
python collect_measurements.py <out_file> <max_size> <num_iterations>