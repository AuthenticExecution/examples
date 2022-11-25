import sys
import json
import numpy as np

file = sys.argv[1]

with open(file, "r") as f:
    data = json.load(f)

averages = {}
for size in data:
    averages[size] = np.mean(data[size])

print(json.dumps(averages, indent=4))