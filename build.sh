#!/usr/bin/env bash
docker run  --volume /Users/dimaakusev/IdeaProjects/sage_controller:/home/cross/project ragnaroek/rust-raspberry:1.39.0 build --release