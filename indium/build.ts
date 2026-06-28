Bun.build({
  entrypoints: ["./src/script.ts"],
  outdir: "./dist",
}).then(() => {
  console.log("Indium built");
});

Bun.write("./dist/index.html", Bun.file("./src/index.html")).then(() => {
  console.log(".html copied");
});

Bun.write("./dist/style.css", Bun.file("./src/style.css")).then(() => {
  console.log(".css copied");
});
