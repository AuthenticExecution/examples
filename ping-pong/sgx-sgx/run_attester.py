import subprocess
import sys
import yaml

ATTESTER = "sgx-attester"
MANAGER_CONFIG = "manager.yaml"
MANAGER_ATTEST_DATA = "manager-attest.yaml"

def attest():
    try:
        res = subprocess.run([ATTESTER, MANAGER_ATTEST_DATA], stdout=subprocess.PIPE)
        res.check_returncode()
    except Exception as e:
        print(e)
        sys.exit(1)

    return eval(res.stdout)


if __name__ == "__main__":
    with open(MANAGER_CONFIG, 'r') as f:
        manager = yaml.load(f, Loader=yaml.FullLoader)

    manager["key"] = attest()

    with open(MANAGER_CONFIG, "w") as f:
        yaml.dump(manager, f)
