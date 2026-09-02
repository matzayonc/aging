import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import styles from './index.module.css';

const pages = [
  {
    title: 'Context',
    to: '/docs/context',
    description:
      'The problem tranching solves, the exposure axis, the vocabulary, and where the prototype currently stands.',
  },
  {
    title: 'The TradFi Implementation',
    to: '/docs/traditional-finance',
    description:
      'How structured credit does this already: waterfalls, attachment points, copula pricing, and institutional price discovery.',
  },
  {
    title: 'Mental Model #1',
    to: '/docs/mental-model-1',
    description:
      'The design being built toward: positions, outright band sales, and a per-band secondary order book pegged to the primary market.',
  },
  {
    title: 'Invariants',
    to: '/docs/invariants',
    description:
      'System-level properties that hold regardless of implementation: value never negative, value conserved against the underlying asset, tranche liquidity balanced.',
  },
  {
    title: 'User Experience',
    to: '/docs/user-experience',
    description:
      'What different users actually see and do: primary deposits, direct order-book access, and the retail leverage slider.',
  },
  {
    title: 'Tranche Pricing Example',
    to: '/docs/tranche-pricing-example',
    description:
      'A fully worked numeric example: normal pricing, an equity wipeout, and how that reprices junior/senior over time.',
  },
];

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout title={siteConfig.title} description={siteConfig.tagline}>
      <header className={clsx('hero hero--primary', styles.heroBanner)}>
        <div className="container">
          <h1 className="hero__title">{siteConfig.title}</h1>
          <p className="hero__subtitle">{siteConfig.tagline}</p>
          <div className={styles.buttons}>
            <Link className="button button--secondary button--lg" to="/docs/context">
              Start with the context
            </Link>
          </div>
        </div>
      </header>
      <main>
        <section className={styles.pages}>
          <div className="container">
            <div className="row">
              {pages.map((page) => (
                <div key={page.to} className="col col--4 margin-bottom--lg">
                  <Link className={styles.pageCard} to={page.to}>
                    <h3>{page.title}</h3>
                    <p>{page.description}</p>
                  </Link>
                </div>
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
