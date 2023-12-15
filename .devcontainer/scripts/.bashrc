#!/bin/bash

repo_root="/workspaces/slang"

# Hermit
eval "$("${repo_root}/bin/hermit" shell-hooks --print --bash)"
