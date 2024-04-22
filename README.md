# Rye Plugin

[![ci](https://github.com/fluentci-io/rye-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/rye-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [rye](https://github.com/astral-sh/rye).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm rye setup
```

## Functions

| Name   | Description                                |
| ------ | ------------------------------------------ |
| setup  | Installs a specific version of rye.       |
| fmt    | Run the code formatter on the project     |
| build  | Builds a package for distribution         |
| fetch  | Fetches a Python interpreter for the local machine |
| init   | Initialize a new or existing Python project with Rye |
| install | Installs a package as global tool |
| lint    | Run the linter on the project |
| publish | Publish packages to a package repository |
| run     | Runs a command installed into this package |
| toolchain | Helper utility to manage Python toolchains |
| tools |  Helper utility to manage global tools |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/rye@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: rye
    args: |
      setup
    working-directory: example
- name: Show rye version
  run: |
    export PATH=${HOME}/.rye/shims:${PATH}
    type rye
    rye --version
```
