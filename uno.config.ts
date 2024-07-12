import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetTypography,
  presetUno,
  presetWebFonts,
} from "unocss";

export default defineConfig({
  rules: [
    [
      /^bg-p-(.*)$/,
      ([, c]) => ({
        "--bg-opacity": 1,
        "background-color": `var(--p-${c})`,
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
