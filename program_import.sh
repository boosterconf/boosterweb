#!/bin/bash

while IFS=, read -r number email navn copresentername copresentermail norsk title type i j k l m n o p q r s t u v w akseptert y z
do
if [ "${akseptert}" == "x" ]
then
    FILENAME=$number-$(echo $title | sed -E 's/[^a-zA-Z]+/-/g' | sed -E 's/[-]+$//' | sed -E 's/^[-]+//' | tr '[:upper:]' '[:lower:]').md
    CLEANED_TITLE=$(echo $title | sed -E 's/[\"]+//g')
echo "---
title: \"$CLEANED_TITLE\"
authors:
    - $navn
    - $copresentername                           
---" > ./content/talk/$FILENAME
fi
done < ./talks.csv