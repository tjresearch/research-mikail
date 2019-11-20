#!/bin/bash
# given to me by John
  
git submodule update --remote --merge

submodules="$(git submodule foreach pwd | grep '^/')"
if [[ ! -z "$(git status --porcelain=v1 -- $submodules)" ]]; then
    echo "Committing and pushing changes..."
    git commit -m "Update submodules" -- $submodules
    git push
fi

