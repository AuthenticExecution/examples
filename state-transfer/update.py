import json
import sys

file = sys.argv[1]
module = sys.argv[2]
node = sys.argv[3]

with open(file, "r") as f:
    conf = json.load(f)

mod = None
for m in conf["modules"]:
    if m["name"] == module:
        mod = m
        break

mod["node"] = node

with open(file, "w") as f:
    json.dump(conf, f, indent=4)