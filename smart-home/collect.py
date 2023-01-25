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
    # TZ
    (
        "TZ AES encryption",
        "tz_handle_output_before_encryption_0: ([0-9]+) us",
        "tz_handle_output_after_encryption_0: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "TZ AES decryption",
        "tz_handle_input_before_decryption_0: ([0-9]+) us",
        "tz_handle_input_before_decryption_0: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "TZ SPONGENT encryption",
        "tz_handle_output_before_encryption_1: ([0-9]+) us",
        "tz_handle_output_after_encryption_1: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "TZ SPONGENT decryption",
        "tz_handle_input_before_decryption_1: ([0-9]+) us",
        "tz_handle_input_before_decryption_1: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "TZ enter TA",
        "tz_remote_output_before_dispatch: ([0-9]+) us",
        "tz_handle_input_before_decryption_[0-9]+: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "TZ exit TA",
        "tz_handle_input_after_handler_0: ([0-9]+) us",
        "tz_remote_output_after_dispatch: ([0-9]+) us",
        MeasurementUnit.US
    ),
    # SGX
    (
        "SGX AES encryption",
        "\[web\] INFO: handle_output_before_encryption: ([0-9]+) us",
        "\[web\] INFO: handle_output_after_encryption: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "SGX AES decryption",
        "\[web\] INFO: handle_input_before_decryption: ([0-9]+) us",
        "\[web\] INFO: handle_input_after_decryption: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "SGX enter enclave",
        "\nremote_output_before_dispatch: ([0-9]+) us",
        "\[web\] INFO: handle_input_before_decryption: ([0-9]+) us",
        MeasurementUnit.US
    ),
    (
        "SGX exit enclave",
        "\[web\] INFO: handle_output_after_encryption: ([0-9]+) us",
        "module_output_before_dispatch: ([0-9]+) us",
        MeasurementUnit.US
    )
]

print("Aggregating logs..")
logs = []
for filename in os.listdir(logs_dir):
    print(filename)
    file_path = os.path.join(logs_dir, filename)

    if os.path.isfile(file_path):
        with open(file_path, "r", errors="replace") as f:
            file_lines = f.readlines()

        logs += list(filter(lambda line : re.search(time_regex, line), file_lines))

# sort and join logs
logs = sorted(logs, key=lambda l : int(l.split()[-2]))
logs = "\n".join(logs)

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
