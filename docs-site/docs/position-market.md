---
sidebar_position: 4
---

# Position Market

How a band actually changes hands. Builds on
[Mental Model #1](./mental-model-1.md) — positions, fixed tranche bands,
and selling a band are introduced there in the abstract; this page is
the specific implementation both an outright sale and a resting listing
run through: one order book per band.

## Order book

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

## Two axes, not one

A plain spot order book is one-dimensional: every order is a single
number on a price axis, bids below the market and asks above it. This
book has two independent axes — **price** and **yield-share** — so an
order is a point in a plane, not a point on a line.

That matters because price alone no longer ranks orders. Two orders at
the same price can carry completely different yield-share, and two
orders with identical yield-share can sit at completely different
prices; a buyer has to weigh both dimensions together, not just scan
down a single column of asks.

Five resting orders on one band, plotted by price and yield-share —
hover a point for its exact numbers:

<div style={{margin: '1.5rem 0'}}>
  <div style={{display: 'flex'}}>
    <div className="orderbook-yticks">
      <span className="orderbook-ytick" style={{top: '0%'}}>100%</span>
      <span className="orderbook-ytick" style={{top: '25%'}}>97.5%</span>
      <span className="orderbook-ytick" style={{top: '50%'}}>95%</span>
      <span className="orderbook-ytick" style={{top: '75%'}}>92.5%</span>
      <span className="orderbook-ytick" style={{top: '100%'}}>90%</span>
    </div>
    <div className="orderbook-plot">
      <div className="orderbook-point" style={{left: '20%', top: '40%', background: '#785ef0'}} title="Order A — price 96%, yield-share 20%" />
      <div className="orderbook-label" style={{left: '20%', top: '40%'}}>A</div>
      <div className="orderbook-point" style={{left: '55%', top: '40%', background: '#785ef0'}} title="Order B — price 96%, yield-share 55%" />
      <div className="orderbook-label" style={{left: '55%', top: '40%'}}>B</div>
      <div className="orderbook-point" style={{left: '10%', top: '10%', background: '#785ef0'}} title="Order C — price 99%, yield-share 10%" />
      <div className="orderbook-label" style={{left: '10%', top: '10%'}}>C</div>
      <div className="orderbook-point" style={{left: '55%', top: '90%', background: '#785ef0'}} title="Order D — price 91%, yield-share 55%" />
      <div className="orderbook-label" style={{left: '55%', top: '90%'}}>D</div>
      <div className="orderbook-point" style={{left: '80%', top: '70%', background: '#785ef0'}} title="Order E — price 93%, yield-share 80%" />
      <div className="orderbook-label" style={{left: '80%', top: '70%'}}>E</div>
    </div>
  </div>
  <div className="orderbook-xticks">
    <span className="orderbook-xtick" style={{left: '0%'}}>0%</span>
    <span className="orderbook-xtick" style={{left: '50%'}}>50%</span>
    <span className="orderbook-xtick" style={{left: '100%'}}>100%</span>
  </div>
  <div style={{display: 'flex', justifyContent: 'space-between', marginTop: '0.6rem'}}>
    <div style={{fontSize: '0.85em', color: 'var(--ifm-color-content-secondary)'}}>↑ price</div>
    <div style={{fontSize: '0.85em', color: 'var(--ifm-color-content-secondary)'}}>yield-share →</div>
  </div>
</div>

| Order | Price | Yield-share |
|---|---|---|
| A | 96% | 20% |
| B | 96% | 55% |
| C | 99% | 10% |
| D | 91% | 55% |
| E | 93% | 80% |

A and B quote the **same price** but B keeps far more of the future
yield — B is only a better deal if that yield turns out to be worth the
difference. B and D quote **the same yield-share** at two different
prices — D is cheaper, but only because it's further from the primary
market's implied value for this slice. Neither pair collapses to a
single "better" order the way two asks at different prices would in a
normal book; which one a buyer wants depends on how much they trust the
underlying yield to keep paying.

## What a free yield-share makes possible

Amount, price, and yield-share being three independent numbers on one
order — rather than yield-share always equalling 100% of whatever
amount is sold — enables two things the protocol doesn't need any extra
machinery to support:

**Looping.** Selling part of a position returns real capital up front —
the price the buyer pays, in the underlying asset — without waiting on
a primary-market withdrawal. That capital can go straight back into the
primary market to open a fresh full-range position, part of which can
again be sold, and so on. Each pass compounds the holder's total
exposure to the underlying yield beyond what their original deposit
alone would support — the same leverage-looping pattern DeFi lending
markets run (borrow against collateral, buy more collateral, repeat),
except here it's driven by repeatedly selling exposure on the position
market rather than by borrowing.

**A principal-less, yield-only claim.** Nothing stops a holder from
selling **all** of a band's amount while setting yield-share to **0%**:
the buyer gets full exposure and principal risk, the seller keeps 100%
of that band's future yield and none of the principal that used to back
it. The result has no principal at all behind it — no exposure to lose,
just a live claim on real yield.

That claim is also the most fragile thing in the system. A normal
position's value degrades toward its principal floor only as yield
stops ([Invariants](./invariants.md)' non-negative-value rule) — a
principal-less yield
claim has no floor to degrade toward, because there's no principal
there in the first place. The instant a band's realized yield falls
short of what's owed, the yield-only claim is the first thing wiped out
— **before** that band's own junior capital takes any loss at all,
however senior or junior the band itself is. It sits one step further
out than the most junior capital in the system: junior principal at
least has a value that can only fall to zero; a principal-less yield
claim can be zeroed by a shortfall that wouldn't even dent the
principal underneath it.

## Worked example

The same position, at three points in time: full range at entry, after
selling the senior tranche outright, and with part of the remainder
resting as an open order on that band's order book (committed, not yet
filled). This time the listing carries real numbers — amount 25, price
97, yield-share 100% — a full transfer of that band's principal and
yield together, the same as an outright sale, just routed through the
order book instead of crossing immediately.

Illustrative annual yields, senior lowest to junior highest, one per
fixed band: **5% · 8% · 14% · 20% · 28%** across 0-50, 50-75, 75-90,
90-95, 95-100 — the same 50/25/15/5/5 split as
[Tranche bands](./mental-model-1.md#tranche-bands). The pool's blended
entry yield is just those rates weighted by band size: 9%.

<div style={{margin: '1.5rem 0'}}>
  <div className="tranche-ticks">
    <span className="tranche-tick" style={{left: '0%'}}>0</span>
    <span className="tranche-tick" style={{left: '50%'}}>50</span>
    <span className="tranche-tick" style={{left: '75%'}}>75</span>
    <span className="tranche-tick" style={{left: '100%'}}>100</span>
  </div>

  <div style={{fontSize: '0.85em', fontWeight: 700, margin: '0.5rem 0 0.3rem'}}>At entry</div>
  <div className="tranche-bar">
    <div className="tranche-band" style={{flexGrow: 100, background: '#6b7280'}} title="held — 0-100, earning the pool's 9% blended yield">held · 9%</div>
  </div>

  <div style={{fontSize: '0.85em', fontWeight: 700, margin: '0.9rem 0 0.3rem'}}>After selling the senior tranche</div>
  <div className="tranche-bar">
    <div className="tranche-band" style={{flexGrow: 50, background: '#d55e00'}} title="sold — 0-50, buyer now earns this band's 5% yield">sold · 5%</div>
    <div className="tranche-band" style={{flexGrow: 50, background: '#6b7280'}} title="held — 50-100, blended yield of the remaining bands">held · 13%</div>
  </div>

  <div style={{fontSize: '0.85em', fontWeight: 700, margin: '0.9rem 0 0.3rem'}}>Listing part of the remainder</div>
  <div className="tranche-bar">
    <div className="tranche-band" style={{flexGrow: 50, background: '#d55e00'}} title="sold — 0-50, buyer now earns this band's 5% yield">sold · 5%</div>
    <div className="tranche-band" style={{flexGrow: 25, background: '#785ef0'}} title="listed — 50-75, amount 25 · price 97 · yield-share 100% · this band's own rate is 8%, still earned by the seller until the order fills">listed · 8%</div>
    <div className="tranche-band" style={{flexGrow: 25, background: '#6b7280'}} title="held — 75-100, blended yield of the remaining bands">held · 18%</div>
  </div>
</div>

| State | Segment | Yield |
|---|---|---|
| At entry | held, 0–100 | 9% blended |
| After selling senior | sold, 0–50 (→ buyer) | 5% |
| | held, 50–100 | 13% blended |
| Listing part of the remainder | sold, 0–50 (→ buyer) | 5% |
| | listed, 50–75 (amount 25 · price 97 · yield-share 100%) | 8% |
| | held, 75–100 | 18% blended |

- **held** (gray) — still part of your position, earning its own share
  of the pool's yield.
- **sold** (red) — transferred outright; the buyer now owns this band's
  principal and yield together, going forward.
- **listed** (blue) — a resting order on that band's order book,
  committed but unfilled — and still earning that band's yield for the
  seller in the meantime. Yield only changes hands, along with the
  principal, once someone crosses the order.

Notice the held segment's blended yield rises at each step — 9%, then
13%, then 18% — purely because each sale peels off a lower-yielding
band and leaves a smaller, riskier remainder behind. Nothing about the
remaining bands' own rates changed; the average of what's left just
went up because the safest, cheapest piece is no longer part of it.

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
- A principal-less yield claim doesn't fit the `(owner, size, exposure
  range)` position shape from [Mental Model #1](./mental-model-1.md) —
  it has size and an owner but no exposure range at all. What data
  structure actually represents it, and can it be resold on its own,
  or does it just live as a note against the buyer's position until it
  pays out or is wiped?
