# Aging

Prototype + design docs for an on-chain tranching protocol: mortgage-style
risk layering (senior/junior/equity-style bands) over a yield-bearing
underlying asset.

## What's here

- `src/` — small Rust prototype (~400 lines). `cargo run` simulates quotes
  (yield/exposure/size) and computes a static snapshot of tranche bands from
  them, writing `expectations.png`.
- `docs-site/` — Docusaurus site (deployed to GitHub Pages). This is where
  the real conceptual/design content lives, not `src/`.

## Docs are ahead of the code

The Rust prototype only computes tranche bands from synthetic quotes
(`compute_tranches` in `src/tranche.rs`). Everything else the docs describe —
positions, ownership, deposits/withdrawals, the secondary order book,
default/wipeout handling — exists only as design prose, not implementation.
Don't assume a doc concept (Position, primary/secondary market, wipeout mode)
maps to real code; check `src/` before claiming something is implemented.

## Docs pages (read in this order)

1. **`docs-site/docs/context.md`** — start here: the problem tranching
   solves, the exposure axis, attachment/detachment, senior/junior, and the
   canonical vocabulary table (exposure, tranche/band, attachment,
   detachment, senior, junior, rate, quote, position). Later pages build on
   this rather than redefining these terms.
2. **`docs-site/docs/traditional-finance.md`** — *optional background, not a
   prerequisite* — how structured credit implements the same idea today:
   SPV/waterfall structure, attachment/detachment pricing, the one-factor
   Gaussian copula, OAS, price discovery (rating agencies, TRACE, Totem,
   evaluated pricing vendors), Basel capital treatment — and a closing table
   mapping each piece onto this prototype, including the honest gaps (no
   loss model, no correlation, no dynamic waterfall). The rest of the docs
   are self-contained without it.
3. **`docs-site/docs/mental-model-1.md`** — core trading mechanism: fixed
   tranche bands (mirrors `BAND_FRACTIONS` in `src/tranche.rs`, 50/25/15/5/5),
   primary market (deposit) vs. secondary market (a per-band order book,
   atomically arbitraged against primary so secondary stays pegged to it).
   Trading is **continuous** — orders cross immediately, no epochs/batching.
4. **`docs-site/docs/invariants.md`** — system-level properties: position
   value never negative; position value conserved 1:1 against the underlying
   asset (deposit/withdraw pact); tranche liquidity balanced (normal mode:
   only balanced deposits accepted; wipeout mode: a default clears one or
   more tranches, survivors keep already-earned yield but forgo *future*
   yield until a new, separate deposit rebuilds the missing tranche).
5. **`docs-site/docs/user-experience.md`** — how different users interact:
   primary depositors get a plain full-range deposit; institutions/market
   makers use the order book directly; retail gets a leverage slider that's
   a UX layer over the same order book, not a separate primitive.
6. **`docs-site/docs/tranche-pricing-example.md`** — a fully worked numeric
   example (a simplified 3-way senior/junior/equity split, not the real
   5-band structure) covering normal pricing, an equity wipeout, and how
   that wipeout reprices junior/senior on the secondary market short- vs.
   long-term.
7. **`docs-site/docs/prior-work.md`** — how five on-chain protocols
   (Strata, Royco Dawn — both live; BarnBridge, Saffron Finance,
   Centrifuge — older or adjacent, two of which abandoned or shut down
   tranching) implement the same senior/junior idea: track record,
   similarities (loss order, recovery-by-redirected-yield, the
   one-size-fits-all problem statement), and where this design goes
   further (five fixed bands vs. everyone's two tranches, a per-band
   order book vs. AMM-or-nothing secondary markets, continuous vs.
   Centrifuge's epoch-batched trading, asset-agnostic vs. per-strategy
   deployments).

## Terminology (doc term → code term)

`context.md` is the canonical vocabulary page (exposure, tranche/band,
attachment, detachment, senior, junior, rate, quote, position) — other pages
link to it rather than redefining those terms. Two more terms map directly
to code, introduced in `mental-model-1.md`:

| Doc term | Code term |
|---|---|
| Fixed band boundaries | `BAND_FRACTIONS` (`src/tranche.rs`) |
| Primary snapshot rate | `TrancheOrder::rate` — a yield rate, distinct from a secondary order's principal-denominated price |
| Position, primary/secondary market, order book | doc-only — no code yet |

## Working conventions

- Docs pages are a living **mental model**, not a frozen spec or changelog —
  revise them in place as the design changes. When a new decision supersedes
  something already written (e.g. the order-book model replacing an earlier
  epoch/double-auction description), edit the existing page rather than
  leaving stale, contradictory content next to the new.
- Established style (see any page in `docs-site/docs/`): frontmatter is just
  `sidebar_position: N`; a single H1 matching the sidebar label; flat H2
  sections; terse declarative prose with `**bold**` for terms being
  introduced; tables for numeric examples. New pages need a link added to
  the footer's "Docs" column in `docs-site/docusaurus.config.js` — the
  sidebar itself is autogenerated from `sidebar_position`, no `sidebars.js`
  change needed.
- After editing docs, verify with `cd docs-site && npm run build`
  (`onBrokenLinks: 'throw'` catches broken links/anchors). `build/` is
  gitignored.
- Numeric worked examples are illustrative ("for example" framing) — check
  new numbers for internal consistency (do totals add up, does risk/discount
  ordering make sense) before publishing. The senior/junior/equity prices in
  `tranche-pricing-example.md` were reworked once already because the first
  draft's discount rates didn't behave.
