#!/usr/bin/env sh

set -e

autoreconf -if 2>&1

./configure 2>&1 

echo "\`\`\`sh" > $GITHUB_STEP_SUMMARY
make -s check >> $GITHUB_STEP_SUMMARY
echo "\`\`\`" >> $GITHUB_STEP_SUMMARY

