#!/usr/bin/env bash
set -euo pipefail

# Jekyll builds to /tmp (see `destination:` in _config.yml — OneDrive can
# interrupt Jekyll's many small writes). Keep this path in sync with that.
BUILD_DIR=/tmp/ross-site-build

/opt/homebrew/bin/bundle exec jekyll build

# The repo lives in a OneDrive folder that forces 700/600 perms, and rsync
# preserves source perms (macOS ships openrsync, which ignores --chmod). The
# build dir is on a normal filesystem, so normalise perms here: Apache needs
# directories traversable (755) and files readable (644), otherwise it returns
# 403 "unable to read htaccess file, denying access to be safe".
chmod -R u=rwX,go=rX "$BUILD_DIR"

rsync -avr -e "ssh -J rossano@and.di.unipi.it" "$BUILD_DIR/" a050143@pages.di.unipi.it:./public_html
