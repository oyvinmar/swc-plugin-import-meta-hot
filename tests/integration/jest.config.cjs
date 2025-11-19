module.exports = {
  transform: {
    "\\.([jt]sx?)$": [
      "@swc/jest",
      {
        jsc: {
          experimental: {
            plugins: [
              [require.resolve("../../swc_plugin_import_meta_hot.wasm"), {}],
            ],
          },
        },
        module: {
          type: "commonjs",
        },
      },
    ],
  },
  testRegex: "(\\.|/)(test)\\.([jt]s)$",
  moduleFileExtensions: ["js", "ts", "jsx", "tsx"],
  moduleDirectories: ["src", "node_modules"],
};
