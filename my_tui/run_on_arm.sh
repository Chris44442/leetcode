#!/bin/bash

source ~/.fpga_config_syniosa2
IP=$SOC_IP_SA2
MEM="target/arm-unknown-linux-gnueabi/debug/my_tui"
MEM_HPS="~/my_tui"

cargo build --release

scp $MEM root@$IP:$MEM_HPS
ssh -t root@$IP './my_tui'

