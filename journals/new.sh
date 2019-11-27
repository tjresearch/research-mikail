#!/bin/sh
mkdir week"$1"
cp template.tex week"$1"/journal.tex
sed "s/journalnum{1}/journalnum{$1}/g" -i week"$1"/journal.tex
