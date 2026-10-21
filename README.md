# Sauti

Open voice data for Kenya's languages, recorded by the people who speak them.

Sauti pays people to record and check short voice clips in Sheng, Kikuyu, Dholuo, Luhya, Kalenjin, Somali and Kenyan English. Each validated clip earns two US cents in USDC, and its consent record and audio fingerprint are written to Solana. The finished dataset is released openly, free for anyone to use.

**Project page:** https://www.keeyow.com/sauti/

**Status:** pre-launch. This repository holds the v0 Solana program, the architecture and the project page. The contributor app, validation service and release pipeline arrive in milestone 1.

## Why

Kenyans move between Sheng, a mother tongue, Swahili and English in a single sentence. Voice tools learn from recorded speech, and for most Kenyan languages the open recordings are few and mostly scripted. Sheng and everyday code-switching barely appear at all. Sauti builds that data in the open and pays the people who make it.

## How a clip becomes data

1. **Sign up with a phone number.** A wallet is created in the browser. Prompts never ask for personal details.
2. **Record or transcribe.** Read short prompts aloud, or type what you hear in someone else's clip.
3. **Three speakers check it.** A clip counts once three other native speakers confirm it is clear and matches its text.
4. **Get paid every week.** Two US cents per validated clip in USDC, plus pay for checking, claimed to the contributor's own wallet.
5. **Released to everyone.** Validated clips join open releases on Hugging Face under CC-BY 4.0, each with a datasheet and a benchmark.

## What goes on-chain

Only fingerprints, consent records and payments. Audio stays in storage, and no personal details are ever written on-chain. A relayer pays rent and fees, so contributors never need SOL.

## Repository

| Path | What it is |
|---|---|
| `programs/sauti` | The Solana program (Anchor), v0 |
| `docs/architecture.md` | How the app, validation, storage, Solana program and releases fit together |
| `site/` | The project page served at keeyow.com/sauti |

## The Solana program (v0)

- `register_contributor(consent_hash)` creates the contributor's account, derived from their wallet, holding a fingerprint of the consent they gave in the app. Rent can be paid by the Sauti relayer.
- `submit_clip(clip_hash, language)` logs a clip's audio fingerprint and language as an on-chain event and counts the contributor's clips.

Next come validation records, USDC reward accrual with weekly claims, and licence revenue splits. See [docs/architecture.md](docs/architecture.md).

**Devnet program ID:** set after the first deploy.

### Build and deploy

With Anchor 0.30.1 and the Solana CLI installed:

```
anchor keys sync
anchor build
anchor deploy --provider.cluster devnet
```

Or open [Solana Playground](https://beta.solpg.io), create an Anchor project, and replace everything below Playground's own `declare_id!` line with everything below the `declare_id!` line in `programs/sauti/src/lib.rs`. Then build and deploy to devnet.

## Milestones

| Milestone | Months | Funding | Outcome |
|---|---|---|---|
| 1. Live and paying | 1 to 3 | USD 20,000 | App and validation live, program on mainnet paying USDC, 2,000 contributors, 150 validated hours |
| 2. First open release | 4 to 6 | USD 25,000 | 8,000 contributors, 600 validated hours, first release on Hugging Face, fine-tuned model released openly with before and after error rates, one licence sale split on-chain |
| 3. Seven languages | 7 to 9 | USD 30,000 | 20,000 contributors, 1,200 validated hours across seven language groups, second release, two paid licence deals, replication kit |

The full plan and budget are on the [project page](https://www.keeyow.com/sauti/).

## Security

This repository is public and holds no secrets or personal data. A secret scan runs on every push, and the project page only deploys when it passes. To report a problem, see [SECURITY.md](SECURITY.md).

## Licences

Code in this repository: [MIT](LICENSE). Dataset releases: CC-BY 4.0.

## Contact

Keeyo Digital Solutions Ltd, Nairobi, Kenya. hello@keeyow.com
