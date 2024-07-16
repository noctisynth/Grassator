import { defineConfig } from 'vitepress';

// https://vitepress.dev/reference/site-config
export default defineConfig({
  lang: 'en-US',
  title: 'Grassator',
  description: 'Secure, modern and fast multi-process downloader',
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    logo: '/logo.png',

    nav: [{ text: 'Home', link: '/' }],

    sidebar: [],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/noctisynth/Grassator' },
    ],

    footer: {
      message:
        'Released under the <a href="https://github.com/noctisynth/Grassator/blob/main/LICENSE-MIT">MIT License</a> and <a href="https://github.com/noctisynth/Grassator/blob/main/LICENSE-AGPL">AGPLv3 License</a>.',
      copyright:
        'Copyright © 2011-present <a href="https://github.com/noctisynth">Noctisynth, Inc.</a>',
    },
  },
  locales: {
    root: {
      label: 'English',
      lang: 'en-US',
    },
    zh: {
      label: '简体中文',
      lang: 'zh-CN',
    },
  },
});
