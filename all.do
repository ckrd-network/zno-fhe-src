#!/usr/bin/env bash
# Exit as soon as there is an error.
set -e

# FILEPATH: /home/hedge/src/zno-fhe-src/all.do
#
# This script finds all files with the extension '.do' within a maximum depth of 2 directories.
# It then removes the '.do' extension from the file names and excludes the current script file.
# Finally, it triggers the 'redo-ifchange' command for each modified file.
#
SELF=$(basename "${0##*/}" .do)
find . -maxdepth 2 -type f -name '*.do' -print0 | \
  xargs -0 echo | \
  sed -e 's/\.do//g' -e "s/\.\/$SELF//g" | \
  xargs redo-ifchange

# Find conflicts between here and the remote.
# Do the merge without touching the index, nor the working tree.
git fetch origin main
if git merge-tree "$(git merge-base FETCH_HEAD main)" main FETCH_HEAD | grep -e '<<<<<<<' -e '>>>>>>>'
then
  echo "Merge conflict. May need manual intervention.  Exiting."
  exit 1
else
  # Looks OK.  Pull changes in for real.
  git pull --set-upstream origin main
fi
git push --set-upstream origin main
