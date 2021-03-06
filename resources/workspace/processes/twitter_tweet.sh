#!/usr/bin/env sh

. processes/subr.sh

PIPELINE_CFG="./pipelines/twitter_tweet.json"
PIPELINE_NAME=$(pipeline_name "$PIPELINE_CFG")
LABEL=$(snake_case "$PIPELINE_NAME")
DATE=$(date +%Y-%m-%d)
LOGFILE="./logs/$LABEL/$DATE.log"

mkdir -p "$(dirname "$LOGFILE")"

export NODE_OPTIONS=--max_old_space_size=4096

doit() {
  "$(npm bin)"/sugarcube \
              -c "$PIPELINE_CFG" \
              -Q query_type:twitter_tweet \
              -D ncube \
              -d
}

echo "Starting the incoming scrape of Twitter tweets."

doit "$ID" 2>&1 | tee -a "$LOGFILE"
