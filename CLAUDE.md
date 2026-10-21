# Rules for working in this repository

This repository is public. Everything committed here is published to the internet for good. Git history keeps deleted files, so a secret that is pushed once counts as leaked even if it is removed later.

## Never commit

- Secrets of any kind: API keys, tokens, passwords, passkeys, private keys, Solana keypairs, wallet recovery phrases, `.env` files, pm2 ecosystem files, `.npmrc` files or git credentials.
- Anything from other Keeyo projects or servers. Do not copy files, code, configuration or values from any other repository, folder or server into this one.
- Server details: IP addresses, SSH key names, server paths, internal hostnames, ports, or the names and layout of other services.
- Personal data: contributors' phone numbers, names or recordings, and any database or log export.

Use placeholders such as `YOUR_API_KEY_HERE` in examples.

## Before every commit

1. Turn on the commit guard once per clone: `git config core.hooksPath .githooks`
2. If `gitleaks` is not installed, download the Linux binary from https://github.com/gitleaks/gitleaks/releases
3. Run `gitleaks dir . --config .gitleaks.toml --redact --no-banner` and stop if it reports anything.
4. Read `git status` and `git diff --cached` and confirm every file belongs to Sauti.

## Deploys

- Every push to `main` runs the secret scan. The page in `site/` deploys to GitHub Pages only when the scan passes.
- Never clone, pull or push this repository from a production server.

## Guardrails

Do not edit `.gitleaks.toml`, `.githooks/`, `.github/`, `.gitignore`, `SECURITY.md` or this file unless Pete asks for that exact change in the conversation.

## Writing style for public files

British English. No em dashes. Do not name Pete's previous employers.
