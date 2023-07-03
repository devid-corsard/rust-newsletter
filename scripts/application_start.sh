#!/bin/bash

sudo chmod -R 777 /home/ubuntu/rustapps/newsletter_cd
cd /home/ubuntu/rustapps/newsletter_cd 
sqlx --database-url "$(cat ../newsletter_dburl)" database create
sqlx --database-url "$(cat ../newsletter_dburl)" migrate run
docker build --output=. --target=binaries .
echo "Build complete" >> build.txt
pm2 start cloud_app --name cloud_app_cd
pm2 save
echo "Started" >> start.txt
