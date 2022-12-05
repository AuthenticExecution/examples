# Supply chain example

## Preliminaries

### Running application

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

### Running scripts

This is needed only when collecting data and/or plotting the results.

```bash
# We assume that we are running a Python 3 (>= 3.6) environment

# Upgrade pip and install dependencies
pip install --upgrade pip
pip install -r scripts/requirements.txt

# install screen (needed for running simulation in the background)
sudo apt-get update && sudo apt-get install screen
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
make start KB=<num_kb>

# (descriptor_shipment.json only) start shipment
make start
```

## Data collection and results

### Sensor data

```bash
# Run simulation (stdout printed to out.log)
#
# <max_size>: maximum size of data in KBs
# <num_iterations>: num of iterations for each run
# <out_file>: desired output file name
make run_sensor SIM_MAX_SIZE=<max_size> SIM_ITERATIONS=<num_iterations> RESULT=<out_file>

# Plot data
#
# <data_file>: output file from the previous command (CSV)
# <out_folder>: output folder where the plots will be saved
make plot RESULT=<data_file> OUT_PLOT_FLD=<out_folder>
```

### Start/end shipment
```bash
# Run simulation (stdout printed to out.log)
#
# <num_iterations>: num of iterations
# <out_file>: output file
make run_shipment SIM_ITERATIONS=<num_iterations> RESULT=<out_file>

# Compute average times
#
# <data_file>: output file from the previous command (CSV)
make shipment RESULT=<data_file>
```