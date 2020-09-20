#!/usr/bin/awk

# initialization stuff
BEGIN {
    # if we're allowed to print lines normally
    shouldprint = 0;
    
    # this tells us if we're in a section header
    sheader = 0;
     
    # this tells us if we're currently formatting a table.
    table = 0;
    
    # this tells us if we are in the header of a table.
    #
    # 0 means no.
    # 1 means yes.
    # 2 means we've done it and we're just waiting to get out of the
    #   header.
    theader = 0;
}

###   LINE INITIALIZATION   ###

{ shouldprint = 1; }

###   PROCESSING THE TOP PART   ###

# matches the second line (should be like "FORBES [NUMBER]").
# no need to interpret the third or fourth line; they always says the
# same thing.
NR == 2 {
    split($0, a, /  +/);
    print "# " a[2];

    delete a;
}

NR == 4 {
    split($0, a, /  +/);
    print "*or* " a[2];

    delete a;
}

# the header part is already taken care of
NR <= 5 {
    shouldprint = 0;
}

# matches those dates at the top of the report; makes them a nice list.
/^Date of/ {
    shouldprint = 0;
    split($0, a, /:/);
    split(a[2], b, /  +/);  
    print "* **" a[1] ":** " b[2];

    delete a;
    delete b;
}

###   SECTION STUFF   ###

# turn section headers into a markdown-style ones and print them.
# I would like to mention that I got this regex in one try.
/^[[:upper:]][[:upper:][:punct:][:blank:]]+\([[:lower:][:punct:][:blank:]]+\)$/ {
    split($0, a, /  +/);
    print "## " a[1] "\n*" a[2] "*";

    sheader = 1;
    delete a;
}

###   TABLE STUFF   ###

# matches horizontal separator lines using dashes.
# this means we are beginning a table.
/^(\+-+)+\+$/ && !table {
    table = 1;
    theader = 1;
}

# the vertical bar-separated table headers already work for markdown.
/^(\|.+)+\|$/ && theader == 1 {
    shouldprint = 1;
}

# so does the body of the table.
!theader && table {
    shouldprint = 1;
}

# this ends off the special table header stuff by printing the markdown
# header and body seperator thing.
/^(\+=+)+\+$/ {
    shouldprint = 0;
    gsub(/=+/, ":---");
    gsub(/\+/, "|");
    print;
    theader = 0;
}

# quick check if we are in forbidden territory
sheader || theader || /END OF THE TREASUROR'S WEEKLY REPORT/{
    shouldprint = 0;
}

# print the line if we should print.
shouldprint {
    print;
}

# matches horizontal separator lines using dashes.
# this means we are beginning a table.
/^(\+-+)+\+$/ && table {
    table = 0;
    theader = 0;
}

# matches a lot of equal signs.
# usually i would put /={72}/ but there's a bug in my awk.
/==+/ {
    sheader = 0;
}
