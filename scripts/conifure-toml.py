import toml
import sys

config_tmp_file = "tmp_config.toml"
config_file = "config.toml"

if len(sys.argv) != 3:
    print("expeted 2 args, username and password")
    exit(-1)
usrname = sys.argv[1]
password = sys.argv[2]

# create congig.toml with passed args
config = toml.load(config_tmp_file, )
config.get("database").update({"username": usrname})
config.get("database").update({"password": password})
config_toml = toml.dumps(config);

# write the new config file
open(config_file, mode="x").write(config_toml)