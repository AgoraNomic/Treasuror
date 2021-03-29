#! /bin/bash

# script to add this report to the report archive
# copies files, then adds an entry to the list of files in the report index
# argument one is m for monthly, w for weekly
# argument two is any stuff you want to append to the filename

REPORTDIR="docs/reports"

case $1 in
    (m) DATE=$(date -u +'%Y-%m');
        REPORTTYPE=monthly;
        REGEXIN="/\* \[....-...\?\]/";;
    (w) DATE=$(date -u +'%F');
        REPORTTYPE=weekly;
        REGEXIN="/\* \[....-..-...\?\]/";;
    (*) echo "first arg should be 'm' or 'w'"; exit;;
esac

FILENAME=$DATE$2
INNERREPORTDIR=$REPORTDIR/$REPORTTYPE

cp $REPORTTYPE.txt $INNERREPORTDIR/fresh.txt
cp $REPORTTYPE.txt $INNERREPORTDIR/$FILENAME.txt

echo $REGEXIN

sed -i "$REGEXIN {
i \
* [$FILENAME]($REPORTTYPE/$FILENAME.txt)
:l
n
b l
}" $REPORTDIR/index.md
