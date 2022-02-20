#! /bin/sh

# script to add this report to the report archive
# copies files, then adds an entry to the list of files in the report index
# argument one is m for monthly, w for weekly
# argument two is any stuff you want to append to the filename

SITEDIR="docs"
REPORTDIR="$SITEDIR/reports"
APIDIR="$SITEDIR/api"

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

echo $2;

case $3 in
    ("") ;;
    (*) FILENAME=$3$2
esac

INNERREPORTDIR=$REPORTDIR/$REPORTTYPE

grep -A2 "Total Buoyancy" weekly.txt | cut -d: -f2 | sed 's/\s//g' > $APIDIR/buoyancy.txt

cp $REPORTTYPE.txt $INNERREPORTDIR/fresh.txt
cp $REPORTTYPE.txt $INNERREPORTDIR/$FILENAME.txt

grep "$FILENAME" $REPORTDIR/index.md >/dev/null || sed -i "$REGEXIN {
i * [$FILENAME]($REPORTTYPE/$FILENAME.txt)
:l
n
b l
}" $REPORTDIR/index.md
