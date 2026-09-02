---
sidebar_position: 3
---

# Mental Model #1

How on-chain tranching works: what a position is, how it's split into
tranches, and how tranches change hands. This describes the trading
mechanism only — where the underlying yield actually comes from (the
mortgage-like asset side) is out of scope here and treated as a black
box, the same way `SimulatedDataProducer` stands in for it in the
current prototype.

## Positions

A user enters by depositing into the pool. Their position spans the
**entire risk spectrum**, 0–100% exposure, sized by their deposit — not
a slice of it. While unsplit, a position earns the pool's blended yield
across that whole range.

A position is, conceptually: `(owner, size, exposure range currently
held)`. On entry, that range is always `[0, 100]`. Selling narrows it.

## Tranche bands

The 0–100% exposure axis is cut into a **fixed, predefined** set of
bands — mortgage-style: a fat, safe tranche at the bottom, then
progressively thinner, riskier bands toward the top. Bottom bands are
senior: first in line to stay whole, last in line for yield. Top bands
are junior: first to absorb losses, first (and biggest) claim on yield.

These bounds aren't chosen per-trade — a seller can exit along one of
these fixed boundaries (e.g. "sell my senior 50%"), not an arbitrary
custom slice. This mirrors `BAND_FRACTIONS` in `src/tranche.rs` today
(`[0.0, 0.5, 0.75, 0.9, 0.95, 1.0]`): a 50% senior tranche, then 25%,
15%, 5%, 5% bands rising in risk.

## Selling a band

Worked example: a user enters on the full `[0, 100]` range, earning
blended yield. They decide to sell their senior tranche — the bottom
50%.

Selling a band transfers it **outright, going forward**: the buyer now
owns that exposure band from that point on — bearing its risk and
receiving its yield. The seller's position shrinks to whatever they
didn't sell — here, the junior `[50, 100]` remainder. Nothing about the
past is undone; this only changes who holds the band from the sale
onward.

## Epochs and the double auction

Trading happens in **epochs** — e.g. one week each. Within an epoch, for
each fixed tranche band:

- **Sellers** who currently hold that band and want out post asks: how
  much yield they're willing to give up to exit.
- **Buyers** post bids: capital, and the yield they require to take on
  that band.

This is a **two-sided (double) auction** — both sides state terms. It's
conceptually close to `compute_tranches`' existing "sort by rate, fill
by size" sweep, generalized from deriving a rate off one-sided quotes
to matching supply against demand.

Settlement only happens at **epoch close**. An order placed mid-epoch is
a commitment, not a fill — nobody knows their executed price, or
whether they're filled at all, until the epoch ends and that band's
orders are cleared together in a batch. There's no continuous trading
within an epoch.

## Worked example

The same position, at three points in time: full range at entry, after
selling the senior tranche outright, and mid-epoch with part of the
remainder listed in the current auction (committed, not yet settled).

```mermaid
%%{init: {'gantt': {'barHeight': 26, 'barGap': 10, 'topPadding': 40, 'fontSize': 13, 'sectionFontSize': 13}, 'themeVariables': {'doneTaskBkgColor': '#6b7280', 'doneTaskBorderColor': '#4b5563', 'activeTaskBkgColor': '#2563eb', 'activeTaskBorderColor': '#1e3a8a', 'critBkgColor': '#dc2626', 'critBorderColor': '#991b1b', 'taskTextColor': '#ffffff', 'taskTextOutsideColor': '#ffffff', 'taskTextLightColor': '#ffffff', 'taskTextDarkColor': '#ffffff'}} }%%
gantt
    title Target exposure (%), 0-100
    dateFormat X
    axisFormat %s

    section At entry
    held           :done, 0, 100

    section After selling the senior tranche
    sold           :crit, 0, 50
    held           :done, 50, 100

    section Auctioning part of the remainder
    sold           :crit, 0, 50
    in auction     :active, 50, 75
    held           :done, 75, 100
```

- **held** (gray) — still part of your position, earning its share of
  yield.
- **sold** (red) — transferred outright; the buyer now owns this band
  going forward.
- **in auction** (blue) — asks/bids posted this epoch, outcome unknown
  until close.

## Open questions

Deliberately unresolved for now — flagged so they don't get silently
assumed:

- What happens to orders left unmatched at epoch close: do they expire,
  roll into the next epoch, or partial-fill?
- How is a single clearing rate computed for a band when its total
  buy-side and sell-side size don't match?
- Can a position be sold down across multiple epochs (partial sales
  layered over time), or is a band's exit an all-or-nothing event per
  position?

## Relationship to the prototype

| Doc term | Code term (`src/tranche.rs`) |
|---|---|
| Exposure axis, 0–100% | `EXPOSURE_MIN`, `EXPOSURE_MAX` |
| Tranche band | `TrancheOrder`, bounded by `attachment`/`detachment` |
| Fixed band boundaries | `BAND_FRACTIONS` |
| Band price | `TrancheOrder::rate` |

The prototype today computes tranches as a static, aggregate snapshot
from many independent `Quote`s — there's no `Position` type, no
ownership, no partial transfer, and no epoch/time dimension yet. This
document is the mental model those pieces would need to be built
against.
