#!/bin/sh
sudo mkdir -p /mnt/pico
sudo mount -t drvfs e: /mnt/pico
elf2uf2-rs -d $1