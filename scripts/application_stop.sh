#!/bin/bash

pm2 stop cloud_app_cd 2> /dev/null
echo "App stopped!" > application_stop.txt
