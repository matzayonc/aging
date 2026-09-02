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
            // Gantt task states, reused across docs. Read them as a
            // three-way status, whatever the diagram is about:
            //   done (gray) -> intact / still held
            //   active (blue) -> partially hit / in flight
            //   crit (red) -> gone / transferred away
            doneTaskBkgColor: '#6b7280',
            doneTaskBorderColor: '#4b5563',
            activeTaskBkgColor: '#2563eb',
            activeTaskBorderColor: '#1e3a8a',
            critBkgColor: '#dc2626',
            critBorderColor: '#991b1b',
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
          themeCSS: `
            .senior > rect, .senior > polygon, .senior > path { fill: #6b7280; stroke: #4b5563; }
            .mezz   > rect, .mezz   > polygon, .mezz   > path { fill: #d97706; stroke: #b45309; }
            .junior > rect, .junior > polygon, .junior > path { fill: #dc2626; stroke: #991b1b; }
            .good   > rect, .good   > polygon, .good   > path { fill: #059669; stroke: #047857; }
            .bad    > rect, .bad    > polygon, .bad    > path { fill: #dc2626; stroke: #991b1b; }
            .step   > rect, .step   > polygon, .step   > path { fill: #2563eb; stroke: #1e3a8a; }
            .senior .nodeLabel, .mezz .nodeLabel, .junior .nodeLabel,
            .good .nodeLabel, .bad .nodeLabel, .step .nodeLabel { color: #ffffff; }
            .senior text, .mezz text, .junior text,
            .good text, .bad text, .step text { fill: #ffffff; }
          `,
        },
      },
    }),
};

module.exports = config;
