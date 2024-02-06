# Create a target

## Install Dependencies
```bash
sudo apt-get install nvme-cli linux-modules-extra-$(uname -r)
sudo modprobe nvme_tcp
sudo modprobe nvmet
sudo modprobe nvmet-tcp
```

## Create Namespace
```bash
sudo mkdir /sys/kernel/config/nvmet/subsystems/test
cd /sys/kernel/config/nvmet/subsystems/test
echo 1 | sudo tee -a attr_allow_any_host > /dev/null
sudo mkdir namespaces/1
cd namespaces/1
sudo echo -n /dev/nvme0n1 | sudo tee -a device_path > /dev/null
echo 1 | sudo tee -a enable > /dev/null
```

## Create Port
```bash
sudo mkdir /sys/kernel/config/nvmet/ports/1
cd /sys/kernel/config/nvmet/ports/1
echo "172.27.22.113" | sudo tee -a addr_traddr > /dev/null
echo tcp | sudo tee -a addr_trtype > /dev/null
echo 4420 | sudo tee -a addr_trsvcid > /dev/null
echo ipv4 |sudo tee -a addr_adrfam > /dev/null
sudo ln -s /sys/kernel/config/nvmet/subsystems/test/ /sys/kernel/config/nvmet/ports/1/subsystems/test
```

# Create an initiator

## Install Dependencies
```bash
sudo apt-get install nvme-cli linux-modules-extra-$(uname -r)
sudo modprobe nvme-tcp
```
## Discover
```bash
sudo nvme discover -t tcp -a 172.27.22.113 -s 4420
```
## Connect
```bash
sudo nvme connect -n test -t tcp -a 172.27.22.113 -s 4420
```

## Verify
```bash
lsblk
```
