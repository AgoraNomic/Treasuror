AWK = awk # the awk implementation to use

ISODATE != date -u "%F"
LONGDATE != date -u "%d %B %Y"

REPORTDIR = docs/reports
WEEKLYDIR = ${REPORTDIR}/weekly
WEEKMDDIR = ${REPORTDIR}/weeklymd

REPORTTXT = ${WEEKLYDIR}/fresh.txt
REPORTMD = ${WEEKMDDIR}/${ISODATE}.md

.PHONY: markdown move date

markdown: fresh.txt
	${AWK} -f markdown.awk fresh.txt > out.md

date:
	sed -i "s/\(whenever\)/${LONGDATE}/" ${REPORTTXT} ${REPORTMD}

move: out.md fresh.txt
	mv fresh.txt ${REPORTTXT}
	mv out.md ${REPORTMD}.md

