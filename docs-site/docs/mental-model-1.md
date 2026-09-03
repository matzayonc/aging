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
**[position market](./position-market.md)**; it never moves the
position sideways.

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

This, like every exit from a band, happens through that band's own
order book: an order that crosses immediately is what an outright sale
looks like; one that doesn't rest as a listing instead. The order book
itself, its order format, and a worked example carrying a position
through it are covered on their own page:
[Position Market](./position-market.md).
