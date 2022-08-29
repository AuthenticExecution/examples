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
    try:
        for line in lines_time:
            measurement = get_measurement_name(line)
            value = float(re.findall(line.format(sm), out)[0])
            RESULTS[sm][measurement] += value
    except:
        print("__fetch_matches failed. Cannot continue.")
        sys.exit(-1)

def fetch_matches(out):
    for sm in SMs:
        __fetch_matches(sm, out)

def compute_iteration(num):   
    print("Starting iteration {}".format(num))
    proc = subprocess.Popen([
        'docker',
        'compose',
        'up', 
        '--abort-on-container-exit',
        '--force-recreate',
        '--exit-code-from',
        'admin'
        ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    res = True
    try:
        out, err = proc.communicate()

        if proc.returncode != 0:
            #print(out.decode('utf-8'))
            print("ERROR: {}".format(proc.returncode))
            res = False
        else:
            print("Iteration ended, fetching data")
            fetch_matches(out.decode('utf-8'))
            print("Iteration complete")
    except Exception as e:
        print("Exception: {}".format(e))
        if proc.returncode == None:
            proc.terminate()
        res = False
    finally:
        proc = subprocess.Popen([
            'docker',
            'compose',
            'rm', 
            '-f',
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        proc.wait()
    
    return res

# init RESULTS
for sm in SMs:
    RESULTS[sm] = {}
    for line in lines_time:
        measurement = get_measurement_name(line)
        RESULTS[sm][measurement] = 0

successes = 0
while successes < NUM_MEASUREMENTS:
    res = compute_iteration(successes)
    if res:
        successes += 1
    else:
        print("Iteration {} failed. Retrying".format(successes))

# compute average
for sm in SMs:
    for line in lines_time:
        measurement = get_measurement_name(line)
        RESULTS[sm][measurement] /= NUM_MEASUREMENTS

with open(output_file, "w") as f:
    json.dump(RESULTS, f, indent=4)

print("Done.")