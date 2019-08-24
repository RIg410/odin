#!/usr/bin/env bash
#ssh pi@192.168.0.100 'sudo systemctl stop home_controller'
scp ./target/arm-unknown-linux-gnueabihf/release/home_controller pi@192.168.0.100:/home/pi/jane/home_controller
#ssh pi@192.168.0.100 'sudo systemctl start home_controller'