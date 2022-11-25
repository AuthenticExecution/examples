import sys
import subprocess
import re
import json

output_file = sys.argv[1]
MAX_SIZE = int(sys.argv[2])
ITERATIONS = int(sys.argv[3])

start_line = ".*START_SENSING: ([0-9]+) ms"
end_line = ".*END_SENSING: ([0-9]+) ms"

results = {}
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
    times = []
    data_start = []
    data_end = []

    for line in out.split("\n"):
        sl = re.findall(start_line, line)
        el = re.findall(end_line, line)

        if sl:
            data_start.append(int(sl[0]))
        elif el:
            data_end.append(int(el[0]))

    if len(data_start) != len(data_end) or len(data_start) != ITERATIONS:
        raise Exception(
            f"Missing data! Expected iterations: {ITERATIONS}" \
                + f" found: {len(data_start)} {len(data_end)}"
        )

    for a,b in zip(data_start, data_end):
        times.append(b - a)

    results[size] = times

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

for size in sizes:
    res = False
    while not res:
        res = compute_iteration(size)

with open(output_file, "w") as f:
    json.dump(results, f, indent=4)

set_env_file(False, size)
print("Done.")