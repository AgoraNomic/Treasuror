# DISCLAIMER: I DO NOT KNOW HOW TO WRITE GOOD MAKEFILES
# DISCLAIMER: THIS MAKEFILE IS ONLY TESTED ON OPENBSD

AWK = awk # the awk implementation to use

ISODATE != date -u +"%F" # unicode date in ISO format
LONGDATE != date -u +"%d %B %Y" # unicode date in better format

REPORTDIR = docs/reports
WEEKLYDIR = ${REPORTDIR}/weekly
WEEKMDDIR = ${REPORTDIR}/weeklymd

REPORTTXT = ${WEEKLYDIR}/fresh.txt
REPORTMD = ${WEEKMDDIR}/fresh.md

.PHONY: markdown date copy

markdown: fresh.txt
	${AWK} -f markdown.awk fresh.txt > out.md

date:
	sed -i "s/(whenever)/${LONGDATE}/" ${REPORTTXT} ${REPORTMD}
	cp ${REPORTTXT} ${WEEKLYDIR}/${ISODATE}.txt
	cp ${REPORTMD} ${WEEKMDDIR}/${ISODATE}.md

copy: out.md fresh.txt
	cp fresh.txt ${REPORTTXT}
	cp out.md ${REPORTMD}

