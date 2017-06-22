#!/bin/bash

openssl req -x509 -new \
    -newkey rsa:2048 \
    -sha384 \
    -nodes \
    -verbose \
    -config config.cnf \
    -extensions "my extensions" \
    -days 7305 \
    -outform pem \
    -out cert.pem \
    -keyout private.pem

