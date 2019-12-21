#!/bin/bash
# https://www.npmjs.com/package/loadtest

# sudo apt install npm
sudo npm install -g loadtest
# Test the GET API
loadtest http://0.0.0.0:8000/api/user/v1/steadylearner -t 20 -c 10 --keepalive --rps 2000 
