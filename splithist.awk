BEGIN {
    # declare field separator for tables
    FS = "|";

    # a variable to track where we are
    # 0 means we're not in the history section
    # 1 means we are in the history section but should not use its tables
    # 4 or more means we are ready to start reading the changelogs
    # -1 means we have hit the last weekly report
    position = 0;
}

/^RECENT HISTORY/ {
    position = 1;
}

/^(\+-+)+\+$/ && position > 0 {
    position++;
}

/WEEKLY/ {
    position = -1;
}

/^(\|.+)+\|$/ && position >= 4 {
    entities[$2] = entities[$2] $0 "\n";
}

END {
    for (i in entities) {
	print "*** ENTITY " i ;
	print entities[i] "\n";
    }
}
