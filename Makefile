THIS_FILE := $(lastword $(MAKEFILE_LIST))

SHELL := $(shell which bash) # set default shell
# OS / Arch we will build our binaries for
#OSARCH := "linux/amd64 linux/386 windows/amd64 windows/386 darwin/amd64 darwin/386"
#ENV = /usr/bin/env

.SHELLFLAGS = -c # Run commands in a -c flag
.SILENT: ; # no need for @
.ONESHELL: ; # recipes execute in same shell
.NOTPARALLEL: ; # wait for this target to finish
.EXPORT_ALL_VARIABLES: ; # send all vars to shell

.PHONY: all # All targets are accessible for user
.DEFAULT: help # Running Make will run the help target

help:
	make -pRrq  -f $(THIS_FILE) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | egrep -v -e '^[^[:alnum:]]' -e '^$@$$'

run:
	cargo run

build:
	cargo build --release && cp ./target/release/cmover ~/dyak/tools