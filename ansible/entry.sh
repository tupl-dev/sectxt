#!/bin/sh
set -eu

KEY_NAME='id_ed25519_sectxt'
ROOT_SSH_DIR='/root/.ssh'

mkdir -p "$ROOT_SSH_DIR"
chmod 700 "$ROOT_SSH_DIR"

if [ -f "/tmp/$KEY_NAME" ]; then
  cp "/tmp/$KEY_NAME" "$ROOT_SSH_DIR/$KEY_NAME"
  chmod 600 "$ROOT_SSH_DIR/$KEY_NAME"
fi

exec "$@"
