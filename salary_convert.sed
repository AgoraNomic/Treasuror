#!/usr/bin/env -S sed -f

# turns Murphy's salary output into valid TLL
# i generally run this through vim over a region, for example with '<,'>!./salary_convert.sed

/\(..\?\) BoC -.\+- \(.\+\) \(assessed\|published the\) \(.\+\)/ {
    s//[XX:XX] \2 \1bl:cn+ "Reward: \4"/
    s/Proposals\? .\+\(.\+\)/Assessed proposal/
    s/proposal pool/Proposal Pool/
}
s/\(.\+\)'s \(.\+\) report/\1 \2/
/^$/d
/^(.\+)$/d
