#!/usr/bin/env bash
docker run  --volume /Users/dimaakusev/IdeaProjects/sage_controller:/home/cross/project ragnaroek/rust-raspberry:1.32.0 build --release