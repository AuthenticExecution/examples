import sys
import subprocess
import re
import json

output_file = sys.argv[1]
NUM_MEASUREMENTS = int(sys.argv[2])

NUM_REGEX = "([0-9\.]+)"

lines_time = [
    f"Build time for {{}}: {NUM_REGEX}",
    f"Deploy time for {{}}: {NUM_REGEX}",
    f"Attest time for {{}}: {NUM_REGEX}",
    f"Connect time for {{}}: {NUM_REGEX}",
    f"Update time for {{}}: {NUM_REGEX}"
]

SMs = [
    "web",
    "gateway",
    "sensor"
]

RESULTS = {}

def init_results():
    res = {}
    res["num_measurements"] = 0
    for sm in SMs:
        res[sm] = {}
        for line in lines_time:
            measurement = get_measurement_name(line)
            res[sm][measurement] = 0

    return res

def store_results():
    results_out = init_results()
    num = RESULTS["num_measurements"]
    results_out["num_measurements"] = num

    # compute average
    for sm in SMs:
        for line in lines_time:
            measurement = get_measurement_name(line)
            results_out[sm][measurement] = RESULTS[sm][measurement] / num

    with open(output_file, "w") as f:
        json.dump(results_out, f, indent=4)

def restore_results():
    global RESULTS

    with open(output_file, "r") as f:
        RESULTS = json.load(f)

    for sm in SMs:
        for line in lines_time:
            measurement = get_measurement_name(line)
            RESULTS[sm][measurement] *= RESULTS["num_measurements"]

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
    print(f"Starting iteration {num + 1}")
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
            print(f"ERROR: {proc.returncode}")
            res = False
        else:
            print("Iteration ended, fetching data")
            fetch_matches(out.decode('utf-8'))
            print("Iteration complete")
    except Exception as e:
        print(f"Exception: {e}")
        if proc.returncode == None:
            proc.terminate()
        res = False
    finally:
        proc = subprocess.Popen([
            'docker',
            'compose',
            'down', 
            ], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        proc.wait()
    
    return res

# init RESULTS
try:
    restore_results()
    print(f"Loaded {RESULTS['num_measurements']} previous iterations")
except:
    print("Starting new evaluation from scratch")
    RESULTS = init_results()

while RESULTS["num_measurements"] < NUM_MEASUREMENTS:
    res = compute_iteration(RESULTS["num_measurements"])
    if res:
        RESULTS["num_measurements"] += 1
        store_results()
    else:
        print(f"Iteration {RESULTS['num_measurements']} failed. Retrying")

print("Done.")