#!/bin/sh

sed -i "2s/.*/$1/" docs/api/buoyancy.txt

cat <<EOF
I flip the Buoyancy Target to the total number of coins given in the Treasuror's report of $(date +"%d %B %Y"), that number being: 

Buoyancy Target: $1
EOF

