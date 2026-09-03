// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const math = require('remark-math');
const katex = require('rehype-katex');

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Aging',
  tagline: 'Mental models and prototypes for on-chain tranching',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://matzayonc.github.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/aging/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'matzayonc', // Usually your GitHub org/user name.
  projectName: 'aging', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  markdown: {
    mermaid: true,
  },

  // KaTeX styles for the math in the structured-finance docs.
  stylesheets: [
    {
      href: 'https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css',
      type: 'text/css',
      integrity:
        'sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM',
      crossorigin: 'anonymous',
    },
  ],
  themes: ['@docusaurus/theme-mermaid'],

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          remarkPlugins: [math],
          rehypePlugins: [katex],
          editUrl: 'https://github.com/matzayonc/aging/tree/master/docs-site/',
        },
        blog: false,
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        title: 'Aging',
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'docsSidebar',
            position: 'left',
            label: 'Docs',
          },
          {
            href: 'https://github.com/matzayonc/aging',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {label: 'Context', to: '/docs/context'},
              {
                label: 'The TradFi Implementation',
                to: '/docs/traditional-finance',
              },
              {label: 'Mental Model #1', to: '/docs/mental-model-1'},
              {label: 'Position Market', to: '/docs/position-market'},
              {
                label: 'Invariants',
                to: '/docs/invariants',
              },
              {
                label: 'User Experience',
                to: '/docs/user-experience',
              },
              {
                label: 'Tranche Pricing Example',
                to: '/docs/tranche-pricing-example',
              },
              {
                label: 'Prior Work',
                to: '/docs/prior-work',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/matzayonc/aging',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Aging. Built with Docusaurus.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
      // Global Mermaid config: merged into mermaid.initialize() for every
      // diagram on the site, so individual blocks don't need %%{init}%%.
      mermaid: {
        theme: {light: 'default', dark: 'dark'},
        options: {
          gantt: {
            barHeight: 26,
            barGap: 10,
            topPadding: 40,
            fontSize: 13,
            sectionFontSize: 13,
          },
          flowchart: {
            curve: 'linear',
            nodeSpacing: 40,
            rankSpacing: 45,
            padding: 10,
          },
          themeVariables: {
            // Gantt task states, reused across docs. Same colors as the
            // flowchart node classes below (done=senior, active=step,
            // crit=junior/bad) so a reader carries one palette across both
            // diagram types, whatever the diagram is about:
            //   done (gray) -> intact / still held
            //   active (indigo) -> partially hit / in flight
            //   crit (vermilion) -> gone / transferred away
            doneTaskBkgColor: '#6b7280',
            doneTaskBorderColor: '#4b5563',
            activeTaskBkgColor: '#785ef0',
            activeTaskBorderColor: '#2d0dc2',
            critBkgColor: '#d55e00',
            critBorderColor: '#843a00',
            taskTextColor: '#ffffff',
            taskTextOutsideColor: '#ffffff',
            taskTextLightColor: '#ffffff',
            taskTextDarkColor: '#ffffff',
          },
          // Node classes for flowcharts, applied in markdown as
          // `N["label"]:::senior` with no local classDef — mermaid puts the
          // bare class name in the SVG and these rules (injected after the
          // built-in ones, scoped per diagram) colour it. Keeping them here
          // means one palette for the whole site instead of a classDef
          // block repeated in every diagram.
          //
          //   senior / mezz / junior -> position in the stack, safe to risky
          //   good / bad             -> which side of a trade-off wins
          //   step                   -> a neutral pipeline stage
          //
          // Colors (besides senior's neutral gray) are drawn from two
          // published colorblind-safe sets rather than hand-picked: mezz,
          // junior/bad and good are Okabe & Ito's 2008 categorical palette
          // (orange, vermilion, bluish green — chosen there specifically to
          // replace pure red/green, the classic protan/deutan collision);
          // step is IBM Design's indigo, from IBM's own color-blind-safe
          // set. Validated pairwise (dataviz skill's validator, OKLab ΔE
          // under Machado-Oliveira-Fernandes CVD simulation): worst
          // deuteranopia pair ΔE 8.5, worst normal-vision pair ΔE 15.2,
          // both clear the accessibility floors — a real fix over the
          // previous hand-tuned amber/red, which measured too close (ΔE
          // 14.4 normal-vision) to reliably tell apart. Okabe-Ito's orange
          // is too light for the shared white node text (2.19:1), so .mezz
          // keeps its own dark-text override.
          themeCSS: `
            .senior > rect, .senior > polygon, .senior > path { fill: #6b7280; stroke: #4b5563; }
            .mezz   > rect, .mezz   > polygon, .mezz   > path { fill: #e69f00; stroke: #8f6300; }
            .junior > rect, .junior > polygon, .junior > path { fill: #d55e00; stroke: #843a00; }
            .good   > rect, .good   > polygon, .good   > path { fill: #009e73; stroke: #006247; }
            .bad    > rect, .bad    > polygon, .bad    > path { fill: #d55e00; stroke: #843a00; }
            .step   > rect, .step   > polygon, .step   > path { fill: #785ef0; stroke: #2d0dc2; }
            .senior .nodeLabel, .junior .nodeLabel,
            .good .nodeLabel, .bad .nodeLabel, .step .nodeLabel { color: #ffffff; }
            .senior text, .junior text,
            .good text, .bad text, .step text { fill: #ffffff; }
            .mezz .nodeLabel { color: #111827; }
            .mezz text { fill: #111827; }
          `,
        },
      },
    }),
};

module.exports = config;
