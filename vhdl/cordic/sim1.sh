#!/bin/bash

python3 run.py
surfer -s=cordic_tb-sim.ron cordic_tb-sim.vcd

# gtkwave calc_tb-sim.vcd wave2.gtkw
# gtkwave cos_1_tb-sim.vcd wave.gtkw

