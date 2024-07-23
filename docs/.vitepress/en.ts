import { defineConfig, type DefaultTheme } from 'vitepress';

export const en = defineConfig({
  lang: 'en-US',
  description: 'Secure, modern and fast multi-process downloader',

  themeConfig: {
    nav: [{ text: 'Home', link: '/' }],

    sidebar: [],

    footer: {
      message:
        'Released under the <a href="https://github.com/noctisynth/Grassator/blob/main/LICENSE-MIT">MIT License</a> and <a href="https://github.com/noctisynth/Grassator/blob/main/LICENSE-AGPL">AGPLv3 License</a>.',
      copyright:
        'Copyright © 2011-present <a href="https://github.com/noctisynth">Noctisynth, Inc.</a>',
    },
  },
});
