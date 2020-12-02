#!/bin/bash
#!/bin/bash

GREEN=$(tput setaf 2)
RED=$(tput setaf 1)
RESET=$(tput sgr0)
IFS=$'\n'

result=($(cargo test | grep -Ei "\[ok\]|\[failed\]"))
formatted=()
for line in "${result[@]}"
do
	line="${line/\[ok\]/$GREEN[ok]$RESET$'\n'}"
	line="${line/\[failed\]/$RED[failed]$RESET$'\n'}"
	formatted+=$line
done

printf "${formatted[*]}" | column -t
