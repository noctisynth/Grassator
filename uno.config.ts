import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetTypography,
  presetUno,
  presetWebFonts,
} from 'unocss';

export default defineConfig({
  rules: [
    [
      /^bg-p-(.*)(\/\d+)?$/,
      ([r, c]) => {
        return {
          '--bg-opacity': 1,
          'background-color': `var(--p-${c})`,
        };
      },
    ],
    [
      /^b-p-(.*)$/,
      ([, c]) => ({
        'border-color': `var(--p-${c})`,
      }),
    ],
    [
      /^text-p-(.*)$/,
      ([, c]) => ({
        color: `var(--p-${c})`,
      }),
    ],
  ],
  presets: [
    presetUno({
      attributifyPseudo: true,
    }),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
    }),
    presetTypography(),
    presetWebFonts(),
  ],
});
