#!/bin/sh
while true; do

  if [ $? -eq 0 ]; then
     
      python /email_sender.py
  else
      echo "Trivy scan failed"
  fi

  sleep 10
done