#!/bin/bash

pm2 start cloud_app -- --name cloud_app_cd
pm2 save
