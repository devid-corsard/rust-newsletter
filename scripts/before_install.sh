#!/bin/bash
# create working directory if it does not exist
DIR="/home/ubuntu/rustapps/newsletter_cd"
if [ -d "$DIR" ]; then
    echo "$DIR exists"
else
    echo "creating $DIR directory"
    mkdir $DIR
fi

