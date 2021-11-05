#!/bin/bash

rustup component add clippy

watch --color cargo clippy --color=always

