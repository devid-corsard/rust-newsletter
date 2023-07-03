#!/bin/bash

cd /home/ubuntu/rustapps/newsletter_cd 
pm2 start cloud_app --name cloud_app_cd
pm2 save
