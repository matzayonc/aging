---
sidebar_position: 3
---

# Mental Model #1

How a position gets split into tranches and how those tranches change
hands. Builds on [Context](./context.md) — the exposure axis, bands,
attachment/detachment and senior/junior are as defined there. Where the
underlying yield actually comes from is treated as a black box here —
out of scope for this page.

## Positions

A user enters by depositing into the pool — the **primary market**. A
position is, concretely: `(owner, size, exposure range currently
held)`. On entry that range is always the whole axis, `[0, 100]`, sized
by the deposit — not a slice of it, and there's no pricing question
here: the deposit *is* the position, 1:1 in the underlying asset. While
unsplit, a position earns the pool's blended yield across that whole
range. Selling only ever narrows the range — selling is the
**position market**, covered below; it never moves the position
sideways.

## Tranche bands

The axis is cut into a **fixed, predefined** set of bands, mortgage-style:
a fat one at the bottom, then progressively thinner ones toward the top.

The bounds aren't chosen per-trade — a seller exits along one of these
preset boundaries (e.g. "sell my senior 50%"), never an arbitrary custom
slice: a 50% senior tranche, then 25%, 15%, 5%, 5% bands rising in risk.

The bar below is the actual axis, to scale, with the real boundaries
marked — hover a band for its exact range.

<div style={{margin: '1.5rem 0'}}>
  <div className="tranche-bar">
    <div className="tranche-band" style={{flexGrow: 50, background: '#6b7280'}} title="Senior — 0-50 (50% of liquidity)" />
    <div className="tranche-band" style={{flexGrow: 25, background: '#a98940'}} title="50-75 (25% of liquidity)" />
    <div className="tranche-band" style={{flexGrow: 15, background: '#e69f00'}} title="75-90 (15% of liquidity)" />
    <div className="tranche-band" style={{flexGrow: 5, background: '#de7f00'}} title="90-95 (5% of liquidity)" />
    <div className="tranche-band" style={{flexGrow: 5, background: '#d55e00'}} title="Junior — 95-100 (5% of liquidity)" />
  </div>
  <div className="tranche-ticks">
    <span className="tranche-tick" style={{left: '0%'}}>0</span>
    <span className="tranche-tick" style={{left: '50%'}}>50</span>
    <span className="tranche-tick" style={{left: '75%'}}>75</span>
    <span className="tranche-tick" style={{left: '90%'}}>90</span>
    <span className="tranche-tick" style={{left: '95%'}}>95</span>
    <span className="tranche-tick" style={{left: '100%'}}>100</span>
  </div>
  <div style={{display: 'flex', justifyContent: 'space-between', marginTop: '0.4rem'}}>
    <div>
      <div style={{fontWeight: 700}}>Senior</div>
      <div style={{fontSize: '0.85em', color: 'var(--ifm-color-content-secondary)'}}>fat, safe, first to stay whole</div>
    </div>
    <div style={{textAlign: 'right'}}>
      <div style={{fontWeight: 700}}>Junior</div>
      <div style={{fontSize: '0.85em', color: 'var(--ifm-color-content-secondary)'}}>thin, risky, first to absorb losses</div>
    </div>
  </div>
</div>

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

## Position market

Each fixed tranche band has its **own, separate order book** — a senior
50% order isn't fungible with a junior 5% order, so their markets don't
mix. What's traded is a **full position**: exposure and yield together,
same as at primary entry — never a stripped-down claim on just one of
the two.

An order is three numbers: **amount, price, and yield-share**.

| Field | Meaning |
|---|---|
| Amount | How much exposure the order covers, in that band. |
| Price | What the buyer pays for it, in the underlying asset. |
| Yield-share | The percentage of the underlying asset's actual yield that goes with this amount, going forward. |

Yield-share is a claim on real yield the underlying produces, not a
rate the maker promises independently of it — buying an order can only
ever entitle you to a *share of* what the asset actually earns, never a
fixed number decoupled from it.

The maker sets price and yield-share freely per order; nothing in the
protocol relates the three fields to each other. There's no pricing
algorithm and no formula — the market, not the system, decides what
yield-share is worth at what price. **Anyone** can place an order, not
just the depositor who originally filled that band: a buyer who fills
an order can immediately turn around and list what they bought.

Because yield-share is now a free variable per order, order-book price
is no longer pinned to what the primary market implies for that slice
— the arbitrage argument that kept the old principal-only order book
pegged to primary no longer applies once yield can be priced
independently of principal. The primary market (deposit at the pool's
blended rate) still exists unchanged alongside this, but it's no
longer a floor or an anchor for what an order can ask.

Order books this way are inherently **discrete** — one per fixed band,
about five of them today — but each one is now itself two-dimensional,
over price and yield-share. Collapsing the five discrete bands into a
single continuous market over risk remains a further, deferred step;
for now, bands stay fixed and each keeps its own order book.

## Worked example

The same position, at three points in time: full range at entry, after
selling the senior tranche outright, and with part of the remainder
resting as an open order on that band's order book (committed, not yet
filled).

<div style={{margin: '1.5rem 0'}}>
  <div className="tranche-ticks">
    <span className="tranche-tick" style={{left: '0%'}}>0</span>
    <span className="tranche-tick" style={{left: '50%'}}>50</span>
    <span className="tranche-tick" style={{left: '75%'}}>75</span>
    <span className="tranche-tick" style={{left: '100%'}}>100</span>
  </div>

  <div style={{fontSize: '0.85em', fontWeight: 700, margin: '0.5rem 0 0.3rem'}}>At entry</div>
  <div className="tranche-bar">
    <div className="tranche-band" style={{flexGrow: 100, background: '#6b7280'}} title="held — 0-100">held</div>
  </div>

  <div style={{fontSize: '0.85em', fontWeight: 700, margin: '0.9rem 0 0.3rem'}}>After selling the senior tranche</div>
  <div className="tranche-bar">
    <div className="tranche-band" style={{flexGrow: 50, background: '#d55e00'}} title="sold — 0-50">sold</div>
    <div className="tranche-band" style={{flexGrow: 50, background: '#6b7280'}} title="held — 50-100">held</div>
  </div>

  <div style={{fontSize: '0.85em', fontWeight: 700, margin: '0.9rem 0 0.3rem'}}>Listing part of the remainder</div>
  <div className="tranche-bar">
    <div className="tranche-band" style={{flexGrow: 50, background: '#d55e00'}} title="sold — 0-50">sold</div>
    <div className="tranche-band" style={{flexGrow: 25, background: '#785ef0'}} title="listed — 50-75">listed</div>
    <div className="tranche-band" style={{flexGrow: 25, background: '#6b7280'}} title="held — 75-100">held</div>
  </div>
</div>

- **held** (gray) — still part of your position, earning its share of
  yield.
- **sold** (red) — transferred outright; the buyer now owns this band
  going forward.
- **listed** (blue) — a resting order on that band's order book,
  committed but unfilled.

## Open questions

Deliberately unresolved for now — flagged so they don't get silently
assumed:

- Do resting orders expire, or stay open indefinitely until filled or
  cancelled?
- What backs market-making on a thin or newly-opened band's order book
  well enough to keep its price reasonable, now that there's no
  primary-market peg to anchor it?
- Can a position be sold down across multiple separate fills (partial
  sales layered over time), or is a band's exit an all-or-nothing event
  per position?
- Can a single position be split across multiple orders with different
  yield-shares — e.g. sell 30% of the yield-share on one order, keep
  the rest for another — or does every order carry 100% of the
  yield-share tied to its amount?
- If yield-share is splittable per order, what stops a maker's
  outstanding orders from collectively promising more yield-share than
  the underlying position actually produces?
