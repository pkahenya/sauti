# Security

## Reporting a problem

Email hello@keeyow.com with the subject "Sauti security". Please do not open a public issue for a security problem.

If you find anything in this repository that looks like a credential, a private key or personal data, report it straight away.

## What this repository holds

Only public code, documentation and the project page. It holds no secrets, no personal data and no audio.

## How secrets are kept out

- A secret scan (gitleaks) runs on every push, every pull request and once a day. The project page only deploys when the scan passes.
- `.gitignore` and a commit guard block key files, environment files, databases and logs.
- GitHub Actions are pinned to exact commits, and Dependabot keeps them and the Rust dependencies up to date.
