#!/bin/bash

lang="$1"
text="$2"

curl -X POST localhost:8001/pastas -H 'Content-Type: application/json' \
	-d '{"lang": "'"$lang"'", "text": "'"$text"'"}'
