s/\(..\) BoC -.\+- \(.\+\) \(assessed\|published the\) \(.\+\)/[XX:XX] \2 \1bl:cn+ "Reward: \4"/
s/\(.\+\)'s \(.\+\) report/\1 \2/
/^$/d
/^(.\+)$/d
