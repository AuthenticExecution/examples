#!/bin/bash

set -e

python collect_measurements.py out.csv 10 100 > out.log 2>&1