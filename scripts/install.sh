#!/bin/bash

sudo chmod -R 777 /home/ubuntu/rustapps/newsletter_cd
cd /home/ubuntu/rustapps/newsletter_cd 
sqlx database create
sqlx migrate run
cargo build --release --bin cloud_app
mv target/release/cloud_app cloud_app
