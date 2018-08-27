#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

set -xe

playbook="$SCRIPTPATH/deploy/site.yml"
inventory="$SCRIPTPATH/deploy/hosts"

currentCommit="$(git rev-parse HEAD | tr -d '[:space:]')"

# NOTE: when setting up a new machine this has to be re-enabled
    # --ask-become-pass \
ansible-playbook \
    --extra-vars "gitsha=$currentCommit" \
    --inventory "$inventory" \
    "$playbook" \
    "$@"
