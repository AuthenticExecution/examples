import json
import csv

input_folder = "tz-sgx"
input_file = "{}/log.txt".format(input_folder)
output_file = "{}/log_parsed.csv".format(input_folder)

lines_time = [
    "[ping] INFO: start:",
    "[ping] INFO: handle_output_before_encryption:",
    "[ping] INFO: handle_output_after_encryption:",
    "module_output_before_dispatch:",
    "Time elapsed in EM:",
    "time before AES decryption in handle_input(router):",
    "time after AES decryption in handle_input(router):",
    "time before SPONGENT encryption in handle_output(router):",
    "time after SPONGENT encryption in handle_output(router):",
    "EM_handle_output:",
    "Time elapsed in EM:",
    "time before SPONGENT decryption in handle_input(router):",
    "time after SPONGENT decryption in handle_input(router):",
    "time before AES encryption in handle_output(router):",
    "time after AES encryption in handle_output(router):",
    "EM_handle_output:",
    "remote_output_before_dispatch:",
    "[ping] INFO: handle_input_before_decryption:",
    "[ping] INFO: handle_input_after_decryption:",
    "[ping] INFO: end:"
]

NUM_MEASUREMENTS   = 14
NUM_RUNS           = 10
NUM_EXPERIMENTS    = 2
TOTAL_MEASUREMENTS = NUM_MEASUREMENTS * NUM_RUNS * NUM_EXPERIMENTS
results = []

with open(input_file, "r") as f:
    lines = f.readlines()

print("Extracting numbers")

# extract and sanitize numbers
for line in lines:
    if any(map(lambda x : x in line, lines_time)):
        val_str = line.replace("us", "").split()[-1]

        # add decimal for measurements in Rust
        if "." not in val_str:
            val_str = val_str[:-6] + "." + val_str[-6:]

        # remove first three digits (excel issue)
        val_str = val_str[3:]

        results.append(float(val_str))

assert(len(results) == TOTAL_MEASUREMENTS)

print("Dividing measurements")

# divide the results
data = {}
csv_data = []
index = 0
for i in range(NUM_EXPERIMENTS):
    exp = {}

    for j in range(NUM_RUNS):
        run = []

        for _ in range(NUM_MEASUREMENTS):
            run.append(results[index])
            index += 1

        csv_data.append([i, j] + run)
        exp[j] = run


    data[i] = exp

assert(len(results) == index)

print("Writing to file")

# print results
with open(output_file, 'w') as csvfile:
    writer = csv.writer(csvfile, delimiter=',',
                    quotechar='|', quoting=csv.QUOTE_MINIMAL)

    writer.writerows(csv_data)

print("Done")
