#!/bin/bash

rm jpas.sqlite && ./jpas-init.sh
echo '{ "secrets" : "foo", "name": "foo", "urls": ["foo", "faa"] }' | ./jpas-set.sh
./jpas-get.sh --name foo
./jpas-get.sh --url foo
./jpas-get.sh --url foo | ./jpas-decrypt.sh