import * as esbuild from "esbuild";

await esbuild.build({
  entryPoints: ["index.css"],
  bundle: true,
  minify: true,
  outdir: "dist",
});
