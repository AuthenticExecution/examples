# Supply chain example

This example is part of the evaluation of the following [academic
publication](https://ieeexplore.ieee.org/document/10382521):

>J. Pennekamp, F. Alder, L. Bader, G. Scopelliti, K. Wehrle and J. T. Mühlberg,
>"Securing Sensing in Supply Chains: Opportunities, Building Blocks, and
>Designs," in IEEE Access, doi: 10.1109/ACCESS.2024.3350778.

Reference outputs are available under [results](./results/). This is also the
same data discussed in the paper.

More info on the code artifacts of this publication are available
[here](https://github.com/COMSYS/secure-sensing).

## Source code

The folder contains two sets of source code:

- `sensor_1.c` and `receiver_1` which contain the code for Sancus (`sensor_1.c`)
  and the accompanying SGX counterpart (`receiver_1`). This part of the code
  implements the sensing and the outputs the data that can be seen in the file
  `results/sensor_data.csv` and is depicted in the Paper as a plot.
- `sensor_2.c` and `receiver_2` which again contain the code for Sancus (`c`
  file) and the SGX part (folder with Rust files). This part of the code
  implements the start/end period of the sensing and its results are stored in
  the file `results/shipment_data.csv`.

## Preliminaries

### Pull docker images and set up credentials and keys

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

### Install dependencies

This is needed only when collecting data and/or plotting the results.

**NOTE** the `tikzplotlib` package has a bug that prevents it from generating
the TEX file  for plots that contain a legend. As of now (version 0.10.1) the
issue has not been solved yet. To circumvent this problem, it may be necessary
to apply the fix discussed in this
[issue](https://github.com/nschloe/tikzplotlib/issues/557).

```bash
# We assume that we are running a Python 3 (>= 3.6) environment

# Upgrade pip and install dependencies
pip install --upgrade pip
pip install -r scripts/requirements.txt

# install screen (needed for running simulation in the background)
sudo apt-get update && sudo apt-get install screen
```

## Run the application

This command deploys the application using Docker Compose and shows the logs in
foreground. This is *not* mandatory to reproduce the results of our paper, as
there is a separate script that manages the whole data collection (see below).

```bash
# Run application
# <descriptor_file>: {descriptor_sensing,descriptor_shipment}.json
make run DESCRIPTOR=<descriptor_file>

# (once finished) close application
CTRL-C
```

It is possible to interact to the application by sending some custom sensor data
or starting shipments, using the commands below:

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

## Reproduce the results of the paper

### Sensing

The command below is used to run the simulation. This may take several minutes
to a few hours to complete.

```bash
# Run simulation (stdout printed to out.log)
#
# <max_size>: maximum size of data in KBs
# <num_iterations>: num of iterations for each run
# <out_file>: desired output file name
make run_sensor SIM_MAX_SIZE=<max_size> SIM_ITERATIONS=<num_iterations> RESULT=<out_file>
```

The command below can be used to create the plot shown in Figure 3 of the paper.
Note that there might be minor differences in the appearance of the plots due to
some parameters that were configured manually.

```bash
# Plot data
#
# <data_file>: output file from the previous command (CSV)
# <out_folder>: output folder where the plots will be saved
make plot RESULT=<data_file> OUT_PLOT_FLD=<out_folder>
```


### Starting and finalizing a shipment

The command below is used to run the simulation. This may take several minutes
to complete.

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