import json
import sys
import os
import re
from enum import IntEnum

logs_dir = sys.argv[1]
output_file = sys.argv[2]

class MeasurementUnit(IntEnum):
    MS = 0
    US = 1

    def to_ms(self, val):
        if self == MeasurementUnit.MS:
            return val
        if self == MeasurementUnit.US:
            return val / 1000
        
        raise Exception(f"Invalid measurement unit: {self.name}")

time_regex = ": [0-9]+ us"

measurements_regexes = [
    (
        "Native encryption",
        "\[ping\] INFO: handle_output_before_encryption: ([0-9]+) us",
        "\[ping\] INFO: handle_output_after_encryption: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "Native decryption",
        "\[ping\] INFO: handle_input_before_decryption: ([0-9]+) us",
        "\[ping\] INFO: handle_input_after_decryption: ([0-9]+) us",
        MeasurementUnit.US
    )
]

print("Aggregating logs..")
all_logs = []
for filename in os.listdir(logs_dir):
    print(filename)
    file_path = os.path.join(logs_dir, filename)

    if os.path.isfile(file_path):
        with open(file_path, "r", errors="replace") as f:
            file_lines = f.readlines()

        all_logs += list(filter(lambda line : re.search(time_regex, line), file_lines))

# sort logs
all_logs = sorted(all_logs, key=lambda l : int(l.split()[-2]))

with open(output_file, "w") as f:
    f.writelines(all_logs)

sys.exit(0)
with open(input_file, "r") as f:
    logs = f.read()

print("Extracting numbers")
results = {}
for measurement in measurements_regexes:
    name = measurement[0]
    start_values = re.findall(measurement[1], logs)
    end_values = re.findall(measurement[2], logs)
    unit = measurement[3]

    if len(end_values) != len(start_values):
        raise Exception(f"Incomplete data for {measurement[0]}: {len(start_values)}/{len(end_values)}")

    # compute elapsed time
    times = []
    for i in range(len(start_values)):
        times.append(float(end_values[i]) - float(start_values[i]))

    results[name] = unit.to_ms(sum(times) / len(times))

# save results
with open(output_file, "w") as f:
    json.dump(results, f, indent=4)

print("Done")
