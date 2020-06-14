BEGIN {
    # this tells us if we need to print a specialized markdown header.
    # I decided not to generate it from the file because I did a lot of squishing there.
    #
    # 0 means no.
    # 1 means yes.
    # 2 means we've done it and we're just waiting to get out of the header.
    header = 0;

    # this represents the groups we need to make separate tables for.
    numgroups = split("Active Player:Zombie:Non-Player Entity", groups, /:/)
    
    # this is the group we're currently in.
    # necessary because some idiot made the final line of the table the same as the line that starts the headers.
    g = 1;
}

# matches horizontal separator lines using dashes.
# this means we need to start to print a header.
/^(\+-+)+\+/ {
    header = 1;
}

# if we're in the body of the table then there's no need to do any special formatting.
!header {
    print;
}

# matches if we need to generate a special header and we haven't printed tables equal to the number of groups.
header == 1 && g <= numgroups {
    # hardcoded special markdown header with all the categories.
    # be quiet, it's not the worst solution.
    blob = "\n|?|Coin|Victory Card|Justice Card|Legislative Card|Voting Card|Victory Point|Blot-B-Gone|Pendant|Extra Vote|";

    # gets the current group name and replaces the header.
    gsub(/\?/, groups[g], blob);
    print blob;

    # variable reinitialization
    header = 2;
    g += 1;
}

# this ends off the special header stuff. 
/^(\+=+)+\+/ {
    sub(/\=+/, ":---------");
    gsub(/\=+/, "---:");
    gsub(/\+/, "|");
    print;
    header = 0;
}
