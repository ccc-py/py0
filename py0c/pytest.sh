set -x

python py0c.py py/hello.py -o qd/hello.qd
python py0c.py py/fact.py -o qd/fact.qd
python py0c.py py/json.py -o qd/json.qd
python py0c.py qd0vm.py -o qd/qd0vm.qd


python qd0vm.py qd/hello.qd
python qd0vm.py qd/fact.qd
python qd0vm.py qd/json.qd
python qd0vm.py qd/qd0vm.qd

