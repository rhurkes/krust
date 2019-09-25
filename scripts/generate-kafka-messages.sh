#!/usr/bin/env bash

seq 100000 | awk '{printf "%s\n", $1}' | kafkacat -P -b 127.0.0.1:9092 -t mytopic
