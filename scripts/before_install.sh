#!/bin/bash
# create working directory if it does not exist
DIR="/home/ubuntu/rustapps/newsletter_cd"
if [ -d "$DIR" ]; then
    echo "$DIR exists"
else
    echo "creating $DIR directory"
    mkdir $DIR
fi

sudo chmod -R 777 /home/ubuntu/rustapps/newsletter_cd
cd /home/ubuntu/rustapps/newsletter_cd 
sqlx --database-url "$(cat ../newsletter_dburl)" database create
sqlx --database-url "$(cat ../newsletter_dburl)" migrate run
export SQLX_OFFLINE=true
cargo build --release --bin cloud_app
mv target/release/cloud_app cloud_app

echo "Build complete" >> build.txt
