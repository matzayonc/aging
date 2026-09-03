---
sidebar_position: 7
---

# Prior Work

Five protocols have shipped some version of on-chain senior/junior
risk-layering. Two are live and structurally close to this design —
**Strata** (`docs.strata.markets`, wraps a single yield strategy like
Ethena's USDe) and **Royco Dawn** (`docs.royco.org`, splits a lending
market, staking deposit, or tokenized RWA). Three are older or adjacent
attempts worth learning from precisely because they didn't all survive
intact: **BarnBridge** (SMART Yield, senior/junior lending-yield
tranches), **Saffron Finance** (AA/A/S tranches over lending protocols),
and **Centrifuge** (DROP/TIN tranches financing real-world assets).
Worth reading alongside [The TradFi Implementation](./traditional-finance.md)
as background, not a prerequisite: this page compares the same
senior/junior idea across five concrete on-chain implementations.

## Track record

Not all of these survived in their original form:

- **Strata** — live, launched October 2025.
- **Royco Dawn** — live.
- **BarnBridge SMART Yield** — **shut down**. The SEC settled an
  enforcement action in December 2023 over unregistered sale of sBOND
  senior tokens as securities (~$509M had flowed through SMART Yield
  pools); BarnBridge DAO and its founders paid ~$1.7M combined, and the
  pools closed. The tranching mechanism itself was never technically
  broken — the shutdown was regulatory, not a design failure.
- **Saffron Finance** — still live, but **abandoned tranching**. The
  original AA/A/S senior/junior product was deprecated in 2021; the
  current "Saffron Vaults" product is a fixed-for-variable swap on
  Uniswap V3 LP fees, not a risk-layered tranche stack.
- **Centrifuge** — live and by far the largest by TVL (>$1B), but
  DROP/TIN tranching is now a **minority feature**. Centrifuge V3
  generalized the fixed senior/junior pair into configurable "share
  classes," and the platform's growth since has come mostly from
  institutional tokenized funds (Anemoy's JTRSY, JAAA, etc.) that don't
  use tranching at all.

Read together: the senior/junior *mechanism* has held up wherever it
shipped — no protocol here lost user funds because the tranche math
broke. What didn't survive was regulatory exposure (BarnBridge) and
product-market fit for tranching specifically, as opposed to the
company built around it (Saffron, and to a lesser extent Centrifuge).

## Similarities

All five converge on the same core primitive independently, which is a
good sanity check on the design:

| Concept | Strata | Royco Dawn | BarnBridge | Saffron (V1) | Centrifuge | This design |
|---|---|---|---|---|---|---|
| Safe layer | Senior Tranche | Senior Tranche (ST) | sBOND (fixed-rate) | AA tranche | DROP | Senior band(s) |
| Risk layer | Junior Tranche | Junior Tranche (JT) | jToken (variable) | A tranche | TIN | Junior band(s) |
| Loss order | Junior absorbs first; senior impaired only once junior is fully depleted | Junior absorbs first, dollar-for-dollar, up to exhaustion | Junior (jToken) drawn down continuously to cover the senior guarantee shortfall | Junior + an insurance fund first; AA "effectively insured" | TIN drawn down first — `seniorAsset = min(expectedSeniorAsset, poolValue)` | Highest-exposure bands absorb first; a wipeout can clear one or more bands entirely |
| Recovery lever | Not described | Observation Period redirects 100% of yield to rebuild the junior buffer | None — continuous absorption, no discrete recovery event | Not documented | Predetermined write-down schedule plus off-chain legal recovery of the underlying asset; no on-chain rebuild | Wipeout mode redirects senior/junior's forgone yield to the deposit that rebuilds the missing band |
| Vocabulary roots | Senior/junior naming, no explicit attachment/detachment | Senior/junior naming, no explicit attachment/detachment | Senior/junior naming, no explicit attachment/detachment | Senior/junior naming, explicit "waterfall" | Senior/junior naming, explicit NAV-based waterfall | Attachment/detachment explicit, borrowed from reinsurance and structured credit — see [Context](./context.md) |
| Primary market | Deposit/redeem through a per-tranche ERC-4626 vault | Deposit/redeem through a per-tranche `RoycoEntryPoint` | Mint jToken anytime; mint an sBOND at a chosen maturity | Deposit into a tranche | Batch orders cleared once per epoch (Tinlake) or synchronous/async vaults (V3) | Full-range `[0, 100]` deposit at the pool's blended rate |

Same problem statement, too: every one of them describes the status quo
as "one blended risk/return forced on every depositor regardless of
appetite" — exactly the complaint [Context](./context.md) opens with.

## Where this design goes further

- **Finer-grained risk axis.** All five prior protocols stop at two
  loss-bearing tranches per market — even Centrifuge's generalized V3
  share classes require a pool manager to configure multiple tranches by
  hand rather than shipping fixed bands as a native primitive. This
  design cuts the same axis into five fixed bands (`BAND_FRACTIONS`, see
  [Mental Model #1](./mental-model-1.md)) — senior depositors can land at
  50%, 75%, 90%, or 95% attachment instead of choosing between exactly
  two points.
- **A real secondary market, not "someone else can build one."** None of
  the five treats secondary liquidity as a protocol-native mechanism.
  Strata's tokens are ERC-4626 shares traded wherever an external DEX
  will list them; Royco gives only senior a dedicated AMM pool; BarnBridge's
  own spec explicitly deferred a senior marketplace to "future work" that
  never shipped before the protocol shut down, leaving jTokens to
  whatever Uniswap pool a third party bothered to create; Saffron's docs
  don't mention a secondary market at all; Centrifuge's DROP/TIN are
  permissioned ERC-20s redeemed back through the pool, not traded
  peer-to-peer. This design gives **every** band its own order book,
  two-dimensional over price and yield-share, atomically arbitraged
  against the primary market.
- **Continuous, not batched.** Centrifuge's Tinlake pools clear orders
  once per **epoch** — a discrete batch auction, not continuous trading.
  This design's order books cross immediately, with no epochs or
  batching (see [Mental Model #1](./mental-model-1.md)).
- **Asset-agnostic by construction.** BarnBridge was scoped to
  lending-protocol yield, Saffron (current) to Uniswap V3 LP fees,
  Centrifuge to real-world assets, Strata and Royco to one deployment per
  underlying strategy — each is either asset-specific or a family of
  asset-specific deployments. This design's mechanism doesn't reference
  what produces the yield at all — see [Context](./context.md)'s
  "asset-agnostic" framing.
- **A continuum, not just two labels.** Because attachment and
  detachment are explicit points on a 0–100 exposure axis rather than
  implicit in a tranche's name, the vocabulary already generalizes past
  two tranches to five — or, later, to a fully continuous market —
  without new terms. Every prior protocol's vocabulary is sized for
  exactly two tranches; adding a third would mean naming it from
  scratch, the way Saffron's V1 had to invent a separate "S" buffer
  tranche on top of AA/A to make its balancing algorithm work.
