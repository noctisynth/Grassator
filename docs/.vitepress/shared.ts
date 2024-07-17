import { defineConfig } from 'vitepress';
import { search as zhSearch } from './zh';

export const shared = defineConfig({
  title: 'Grassator',

  lastUpdated: true,
  cleanUrls: true,
  metaChunk: true,

  sitemap: {
    hostname: 'https://grassator.noctisynth.org',
  },

  themeConfig: {
    logo: {
      dark: '/icon.png',
      light: '/light.ico',
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/noctisynth/Grassator' },
    ],

    search: {
      provider: 'algolia',
      options: {
        appId: '9RCKXREJL8',
        apiKey: 'fed8c71cf1f3731cd91d99fb8b5bc289',
        indexName: 'grassator',
        locales: { ...zhSearch },
      },
    },
  },
});
