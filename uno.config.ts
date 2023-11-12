import { defineConfig, presetIcons, presetUno } from "unocss";

export default defineConfig({
  presets: [
    presetUno(),
    presetIcons({
      collections: {
        fluent: () =>
          import("@iconify-json/fluent-emoji").then((i) => i.icons as any),
      },
    }),
  ],
});
