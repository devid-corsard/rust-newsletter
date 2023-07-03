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
sqlx database create --database-url "postgres://ubuntu:password123@localhost:5432/newsletter"
sqlx migrate run --database-url "postgres://ubuntu:password123@localhost:5432/newsletter"
cargo build --release --bin cloud_app
mv target/release/cloud_app cloud_app

echo "Build complete" > build.txt
