#! /bin/bash

# script to add this report to the report archive
# argument one is any content you want to append to the filename

REPORTDIR='docs/reports/weekly'

cp fresh.txt $REPORTDIR/fresh.txt
cp fresh.txt $REPORTDIR/$(date -u +'%F')$1.txt
