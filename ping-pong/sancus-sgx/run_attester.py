import subprocess
import sys
import yaml

ATTESTER       = "sgx-attester"
manager_config = sys.argv[1] if len(sys.argv) > 1 else "manager.yaml"

def attest(manager):
    env = {
        "AESM_HOST" : "aesm-client",
        "AESM_PORT" : "13741",
        "ENCLAVE_HOST" : manager["host"],
        "ENCLAVE_PORT" : str(manager["port"]),
        "SP_PRIVKEY" : "cred/manager_privkey.pem",
        "ENCLAVE_SIG" : "cred/manager.sig",
        "IAS_CERT" : "cred/ias_root_ca.pem",
        "ENCLAVE_SETTINGS" : "cred/settings.json",
    }

    try:
        res = subprocess.run([ATTESTER], env=env, stdout=subprocess.PIPE)
        res.check_returncode()
    except Exception as e:
        print(e)
        sys.exit(1)

    return eval(res.stdout)


if __name__ == "__main__":
    with open(manager_config, 'r') as f:
        manager = yaml.load(f, Loader=yaml.FullLoader)

    manager["key"] = attest(manager)

    with open(manager_config, "w") as f:
        yaml.dump(manager, f)
