#!/bin/bash
notify_pid="$(pgrep -f /gatodown)"
if [ -z "$notify_pid" ]; then
  cd /opt/magnolia/notify/
  mkdir -p log
  exec 0</dev/null 1>>./log/notify.log 2>&1
  RUST_LOG=info nohup ./gatodown &
else
  echo "A version of gatodown notify application is already running. Exiting out of gatodown notify startup process."
fi
