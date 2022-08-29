import sys
import subprocess
import re
import json

output_file = sys.argv[1]
NUM_MEASUREMENTS = int(sys.argv[2])

NUM_REGEX = "([0-9\.]+)"

lines_time = [
    "Build time for {{}}: {}".format(NUM_REGEX),
    "Deploy time for {{}}: {}".format(NUM_REGEX),
    "Attest time for {{}}: {}".format(NUM_REGEX),
    "Connect time for {{}}: {}".format(NUM_REGEX),
    "Update time for {{}}: {}".format(NUM_REGEX)
]

SMs = [
    "source",
    "gw",
    "sink"
]

RESULTS = {}

def get_measurement_name(line):
    return line.split(" ")[0]

def __fetch_matches(sm, out):
    global RESULTS

    for line in lines_time:
        measurement = get_measurement_name(line)
        value = float(re.findall(line.format(sm), out)[0])
        RESULTS[sm][measurement] += value

def fetch_matches(out):
    for sm in SMs:
        __fetch_matches(sm, out)

def compute_iteration(num):   
    print("Starting iteration {}".format(num))
    proc = subprocess.Popen([
        'docker-compose',
        'up', 
        '--abort-on-container-exit',
        '--exit-code-from',
        'admin'
        ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    try:
        out, err = proc.communicate()

        if proc.returncode != 0:
            print("ERROR! return code: {}".format(proc.returncode))
            return

        print("Iteration ended, fetching data")
        fetch_matches(out)

        print("Iteration complete")
    except Exception as e:
        print("Exception: {}".format(e))
        if proc.returncode == None:
            proc.terminate()

# init RESULTS
for sm in SMs:
    RESULTS[sm] = {}
    for line in lines_time:
        measurement = get_measurement_name(line)
        RESULTS[sm][measurement] = 0

for i in range(NUM_MEASUREMENTS):
    compute_iteration(i)

# compute average
for sm in SMs:
    for line in lines_time:
        measurement = get_measurement_name(line)
        RESULTS[sm][measurement] /= NUM_MEASUREMENTS

with open(output_file, "w") as f:
    json.dump(RESULTS, f, indent=4)

print("Done.")