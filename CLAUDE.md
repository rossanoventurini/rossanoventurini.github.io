# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Rossano Venturini's personal academic website, built on the [al-folio](https://github.com/alshedivat/al-folio) Jekyll theme. Most files come from the upstream theme; the content that actually changes lives in a handful of places (see "Where content lives"). The README.md is the upstream theme's generic documentation — ignore it for day-to-day edits and rely on this file instead.

Live site: `https://pages.di.unipi.it/rossano` (note the `baseurl: /rossano` in `_config.yml` — all internal links must account for it).

## Build & serve

```bash
bundle install                       # one-time / after Gemfile changes
bundle exec jekyll serve --lsi       # local dev server with live reload; --lsi enables related-posts (LSI) indexing
bundle exec jekyll build             # one-off build
```

Docker alternative (matches the README): `docker-compose up` serves on port 8080.

### Build output goes to /tmp, not _site/

`_config.yml` sets `destination: /tmp/ross-site-build`. This is deliberate: the repo lives inside a OneDrive-synced folder, and OneDrive can interrupt the many small writes a Jekyll build performs. **The site is built into `/tmp/ross-site-build`, not the in-repo `_site/`.** Keep this in mind when inspecting build output or pointing tooling at the generated HTML. Note that `PUBLISH_ME.sh` still rsyncs from `_site/`, so it is inconsistent with this destination — verify the path before relying on it to publish.

## Deployment

Two independent paths exist:

- **GitHub Actions** (`.github/workflows/deploy.yml`): on push to `master`/`main`, builds with Ruby 3.2.2 + Jupyter + mermaid CLI and deploys `_site` to the `gh-pages` branch.
- **`PUBLISH_ME.sh`**: manual rsync to the University of Pisa server (`pages.di.unipi.it`) over an SSH jump host (`and.di.unipi.it`). This is the real public target for the `/rossano` URL.

## Where content lives

Editing the actual site means touching these, not the theme internals:

- **`_posts/*.md`** — the "Notes" blog: tutorial-style posts on algorithms, data structures, code optimization, and Rust (e.g. Fenwick trees, prefix sums, Mo's algorithm). Filenames are `YYYY-MM-DD-slug.md`. Front matter uses `tags`, `categories: notes`, a `thumbnail`, and `giscus_comments: true`. Posts use `$$...$$` MathJax and footnotes. The `handson*` posts are course exercise write-ups.
- **`_bibliography/*.bib`** — publications. `papers.bib` is the one Jekyll Scholar renders (configured under `scholar:` in `_config.yml`); `conferences.bib`, `journals.bib`, `chapters.bib` are organizational. The publications page is generated from BibTeX — do not hand-edit publication HTML. Custom BibTeX keywords (`abbr`, `selected`, `pdf`, `code`, `arxiv`, `bibtex_show`, …) control the buttons/badges shown per entry; `selected={true}` flags papers for the selected-papers list.
- **`_pages/*.md`** — top-level pages: `about.md` (the homepage, `permalink: /`), `publications.md`, `courses.md`, `competitive.md`, `dropdown.md`.
- **`_data/`** — `cv.yml` and `assets/json/resume.json` feed the CV (JSON Resume format, pulled via `jekyll-get-json`); `coauthors.yml` links co-author names to their pages in publication listings; `venues.yml` maps `abbr` codes to venue links; `repositories.yml` drives the GitHub stats page.
- **`assets/`** — images, PDFs (`assets/pdf/`), and per-post asset folders (e.g. `assets/img/fenwick/`).

## Custom plugins (`_plugins/`)

Ruby plugins beyond the standard gem list: `cache-bust.rb`, `details.rb`, `external-posts.rb`, `file-exists.rb`, `hideCustomBibtex.rb`. Check these before assuming a Liquid tag or behavior comes from a gem.

## Pre-commit

`.pre-commit-config.yaml` runs trailing-whitespace, end-of-file-fixer, check-yaml, and check-added-large-files. Run `pre-commit run --all-files` if hooks are installed.
