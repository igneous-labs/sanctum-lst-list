# Sanctum LST List

Definitive list of Liquid Staking Tokens integrated under Sanctum.

# Sanctum LST List

---

Definitive list of Liquid Staking Tokens integrated under Sanctum.

## Open a PR for validation:

- Open a PR like [this sample PR](https://github.com/igneous-labs/sanctum-lst-list/pull/38) with your addition in the sanctum-lst-list.toml file.
- After reviewing your PR will be merged and the LST added to the UI

---

## Validated LST list

### Example fields

- `name`: Power Staked SOL
- `symbol`: pwrSOL
- `mint`: pWrSoLAhue6jUxUkbWgmEy5rD9VJzkFmvfTDV5KgNuu
- `decimals`: 9
- `token_program`: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
- `logo_uri`: https://arweave.net/vmJI1aPZNfTTIWH7ZLFxBP1VK7ptapg1hBukoDDNPME
- the `pool` object:
    - `program`: SanctumSpl
    - `pool`: DfiQgSvpW3Dy4gKfhtdHnWGHwFUrE8exvaxqjtMtAVxk
    - `validator_list`: 9PYGjoQhY89f8t3NVtq82MN91RDvoEkkFtcQyuVAuWGL
    - `vote_account`: LodezVTbz3v5GK6oULfWNFfcs7D4rtMZQkmRjnh65gq

Upon creating the token you will get the first 5 fields

The deployer of the pool will have `program` (either SanctumSpl or Spl) and `pool` (public address)

If the pool delegates to a single validator, the validator will have `validator_list` and `vote_account`
