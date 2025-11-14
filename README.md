# swc-plugin-import-meta-hot

> Simple plugin to transform `import.meta.hot` to `process.env`

This `@swc` plugin provides a simple transformation from `import.meta.hot` to `module.hot`.


## Install 🌱

```shell
npm i -D swc-plugin-import-meta-hot
```

## Usage 🚀

Simply add this to the plugins field of your `.swcrc`.

```json
{
  "jsc": {
    "experimental": {
      "plugins": [["swc-plugin-import-meta-hot", {}]]
    }
  }
}
```

Or programmatically as an extension to your existing `.swcrc` parsing:

```js
const swcrc = JSON.parse(fs.readFileSync(".swcrc", "utf8"));
((swcrc.jsc ??= {}).experimental ??= {}).plugins = [
  ["swc-plugin-import-meta-hot", {}],
]; // This may need updating to suit your requirements
```
