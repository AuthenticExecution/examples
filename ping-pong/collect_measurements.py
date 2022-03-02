import json
import csv
import sys
import distutils.util

input_file = sys.argv[1]
output_file = sys.argv[2]
NUM_MEASUREMENTS = int(sys.argv[3])
APPEND_INDEX = distutils.util.strtobool(sys.argv[4]) if len(sys.argv) >= 5 else False

new_iteration_msg = "STARTING NEW ITERATION"
all_done_msg = "ALL DONE"

lines_time = [
    #"[ping] INFO: start:",
    "[ping] INFO: handle_output_before_encryption:",
    "[ping] INFO: handle_output_after_encryption:",
    #"module_output_before_dispatch:",
    #"Time elapsed in EM:",
    "time before AES decryption in handle_input(router):",
    "time after AES decryption in handle_input(router):",
    "time before SPONGENT encryption in handle_output(router):",
    "time after SPONGENT encryption in handle_output(router):",
    #"EM_handle_output:",
    #"Time elapsed in EM:",
    "time before SPONGENT decryption in handle_input(router):",
    "time after SPONGENT decryption in handle_input(router):",
    "time before AES encryption in handle_output(router):",
    "time after AES encryption in handle_output(router):",
    #"EM_handle_output:",
    #"remote_output_before_dispatch:",
    "[ping] INFO: handle_input_before_decryption:",
    "[ping] INFO: handle_input_after_decryption:",
    #"[ping] INFO: end:"
]

def compute_iteration(chunk):
    results = []

    try:
        # extract and sanitize numbers
        for line in chunk:
            if any(map(lambda x : x in line, lines_time)):
                val_str = line.replace("us", "").split()[-1]

                # add decimal for measurements in Rust
                if "." not in val_str:
                    val_str = val_str[:-6] + "." + val_str[-6:]

                # remove first three digits (excel issue)
                val_str = val_str[3:]

                results.append(float(val_str))

        if len(results) != NUM_MEASUREMENTS:
            print("WARNING iteration {}: number of measurements differ".format(it))
            return None

        return results
    except Exception as e:
        print("WARNING iteration {}: {}".format(it, e))
        return None

with open(input_file, "r") as f:
    lines = f.readlines()

print("Extracting numbers")

chunk_indexes = [i for i in range(len(lines)) if new_iteration_msg in lines[i] or all_done_msg in lines[i]]

iterations = []
it = 1
start_i = chunk_indexes[0]
for end_i in chunk_indexes[1:]:
    results = compute_iteration(lines[start_i:end_i])

    if results is not None:
        if APPEND_INDEX:
            iterations.append([it] + results)
        else:
            iterations.append(results)

    it += 1
    start_i = end_i

print("Total: {} iterations".format(len(iterations)))

# print results
with open(output_file, 'w') as csvfile:
    writer = csv.writer(csvfile, delimiter=',',
                    quotechar='|', quoting=csv.QUOTE_MINIMAL)

    writer.writerows(iterations)

print("Done")
