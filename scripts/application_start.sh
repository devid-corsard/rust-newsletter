#!/bin/bash

sudo chmod -R 777 /home/ubuntu/rustapps/rust-newsletter
cd /home/ubuntu/rustapps/rust-newsletter
sqlx database create
sqlx migrate run
cargo build --release --bin cloud_app
mv target/release/cloud_app cloud_app
cargo clean
pm2 start cloud_app
