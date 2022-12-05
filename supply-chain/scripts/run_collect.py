import sys
import subprocess
import re
import pandas as pd
import shutil

output_file = sys.argv[1]
MAX_SIZE = int(sys.argv[2])
ITERATIONS = int(sys.argv[3])
IS_SENSOR = True if int(sys.argv[4]) != 0 else False

if IS_SENSOR:
    start_line = ".*START_SENSING: ([0-9]+) ms"
    transm_line = ".*END_TRANSMISSION: ([0-9]+) ms"
    end_line = ".*END_SENSING: ([0-9]+) ms"
    shutil.copyfile("descriptor_sensing.json", "descriptor.json")
else:
    start_line = ".*START_SHIPMENT: ([0-9]+) ms"
    transm_line = ".*END_TRANSMISSION: ([0-9]+) ms"
    end_line = ".*START_SHIPMENT_COMPLETE: ([0-9]+) ms"
    shutil.copyfile("descriptor_shipment.json", "descriptor.json")

results = []
sizes = range(1, MAX_SIZE+1)

def set_env_file(collect, size=0):
    env_file = [
        "# Do not change. Will be updated automatically",
        f"COLLECT_DATA={int(collect)}",
        f"DATA_SIZE={size}",
        f"ITERATIONS={ITERATIONS}"
    ]

    with open(".env", "w") as f:
        f.write("\n".join(env_file))

def fetch_matches(size, out):
    data_start = []
    data_transm = []
    data_end = []

    for line in out.split("\n"):
        sl = re.findall(start_line, line)
        etl = re.findall(transm_line, line)
        el = re.findall(end_line, line)

        if sl:
            data_start.append(int(sl[0]))
        elif etl:
            data_transm.append(int(etl[0]))
        elif el:
            data_end.append(int(el[0]))

    if len(data_start) != len(data_end) or \
        len(data_start) != len(data_transm) or len(data_start) != ITERATIONS:
        raise Exception(
            f"Missing data! Expected iterations: {ITERATIONS}" \
                + f" found: {len(data_start)} {len(data_transm)} {len(data_end)}"
        )

    for a,b,c in zip(data_start, data_transm, data_end):
        results.append(
            {
                "size": size,
                "sancus": b - a,
                "sgx": c - b,
                "total": c - a
            }
        )

def compute_iteration(size):
    set_env_file(True, size)

    print("Starting run with data size: {}".format(size))
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
        out, _ = proc.communicate()

        if proc.returncode != 0:
            #print(out.decode('utf-8'))
            print("ERROR: {}".format(proc.returncode))
            res = False
        else:
            print("Iteration ended, fetching data")
            fetch_matches(size, out.decode('utf-8'))
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

print(f"Starting simulation. Max size: {MAX_SIZE} iterations: {ITERATIONS}")

for size in sizes:
    res = False
    while not res:
        res = compute_iteration(size)

# create dataframe
df = pd.DataFrame(results)
df.to_csv(output_file)

set_env_file(False, size)
print("Done.")