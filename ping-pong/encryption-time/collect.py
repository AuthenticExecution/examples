import json
import sys
import re
from enum import IntEnum

input_file = sys.argv[1]
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

# TODO add TZ regexes
measurements_regexes = [
    (
        "SGX/native encryption",
        "\[ping\] INFO: handle_output_before_encryption: ([0-9]+) us",
        "\[ping\] INFO: handle_output_after_encryption: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "SGX/native decryption",
        "\[ping\] INFO: handle_input_before_decryption: ([0-9]+) us",
        "\[ping\] INFO: handle_input_after_decryption: ([0-9]+) us",
        MeasurementUnit.US
    )
]

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
