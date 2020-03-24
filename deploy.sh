#!/usr/bin/env bash

ssh pi@192.168.0.100 <<'ENDSSH'
  cd /home/pi/jane/sage_controller
  rm -rf src
  rm Cargo.lock
  rm Cargo.toml
ENDSSH

scp -r ./src pi@192.168.0.100:/home/pi/jane/sage_controller/
scp  ./Cargo.lock pi@192.168.0.100:/home/pi/jane/sage_controller/
scp  ./Cargo.toml pi@192.168.0.100:/home/pi/jane/sage_controller/

ssh pi@192.168.0.100 <<'ENDSSH'
  cd /home/pi/jane/sage_controller
  cargo build --release
  sudo systemctl stop home_controller
  rm /home/pi/jane/home_controller
  cp target/release/home_controller /home/pi/jane/home_controller
  sudo systemctl start home_controller
ENDSSH