#!/bin/bash

###################################################################
#Script Name	: build-linux.sh
#Description	: Builds a release version of the app for linux x64 systems using docker
#Args         :
#Author       : Krakaw
#Email        : 41575888+Krakaw@users.noreply.github.com
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/rrenamer -w /usr/src/rrenamer rust:1.53.0 cargo build --release
