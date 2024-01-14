// unocss.config.ts
import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
} from "unocss";
import { presetIkun, getCSSPreflights, getSafeList } from "@ikun-ui/preset";

export default defineConfig({
  presets: [presetUno(), presetAttributify(), presetIcons(), presetIkun()],
  safelist: [...getSafeList()],
  preflights: [
    {
      layer: "base",
      getCSS: () => `:root {${getCSSPreflights()}}`,
    },
  ],
  rules: [
    [
      "x-middle",
      { position: "absolute", left: "50%", transform: "translateX(-50%)" },
    ],
    [
      "y-middle",
      { position: "absolute", top: "50%", transform: "translateY(-50%)" },
    ],
    [
      "absolute-middle",
      {
        position: "absolute",
        left: "50%",
        top: "50%",
        transform: "translate(-50%, -50%)",
      },
    ],
  ],
});
