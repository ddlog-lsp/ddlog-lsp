<div align="center">
  <h1><code>ddlog-lsp</code></h1>
  <p>
    <strong>A language server implementation for DDlog</strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://ddlog-lsp.github.io/ddlog-lsp/ddlog_lsp_server"><img
        src="https://img.shields.io/badge/docs-latest-blueviolet?logo=Read-the-docs&logoColor=white"
        /></a>
    <a href="https://github.com/ddlog-lsp/ddlog-lsp/actions"><img
        src="https://github.com/ddlog-lsp/ddlog-lsp/workflows/main/badge.svg"
        /></a>
    <a href="https://codecov.io/gh/ddlog-lsp/ddlog-lsp"><img
        src="https://codecov.io/gh/ddlog-lsp/ddlog-lsp/branches/main/graph/badge.svg"
        /></a>
  </p>
</div>

## Status

The server is still in an early state. It is usable but many advanced features have not yet been implemented.

## Usage

The server has not yet had a stable release. You can build and install it locally if you would like to experiment with it in the meantime.

### Installing the Server

#### Prebuilt Binaries

The easiest way to install the server is to grab one of the prebuilt binaries under [releases](https://github.com/ddlog-lsp/ddlog-lsp/releases).

#### Building from Source

First ensure you have the [rust toolchain](https://rustup.rs/) installed, then proceed as follows:

```bash
git clone https://github.com/ddlog-lsp/ddlog-lsp
cd ddlog-lsp
cargo xtask init
cargo xtask install
```

##### Selecting the Async Runtime

The server is runtime agnostic and can be configured to run on [`async-std`](https://github.com/async-rs/async-std), [`futures`](https://github.com/rust-lang/futures-rs), [`smol`](https://github.com/smol-rs/smol), or [`tokio`](https://github.com/tokio-rs/tokio).

The table below describes how to select a runtime. The `tokio` runtime is selected by default.

| runtime     | command                                   |
| ----------- | ----------------------------------------- |
| `async-std` | `cargo xtask install --runtime=async-std` |
| `futures`   | `cargo xtask install --runtime=futures`   |
| `smol`      | `cargo xtask install --runtime=smol`      |
| `tokio`     | `cargo xtask install --runtime=tokio`     |

### Installing the Client Extension

Once the server is installed you can install the Visual Studio Code [client extension](https://github.com/ddlog-lsp/vscode-ddlog).

## Language Server Feature Support

- 🗹 document parsing via [ddlog tree-sitter grammars](https://github.com/ddlog-lsp/tree-sitter-ddlog)
- 🗹 document symbol provider
- 🗹 syntax error diagnostics provider
- 🗹 incremental document synchronization

## Language Server Feature Roadmap

- ☐ code action provider
- ☐ code lens provider
- ☐ completion provider
- ☐ definition provider
- ☐ document formatting (full and ranged) provider
- ☐ document highlight provider
- ☐ hover provider
- ☐ references provider
- ☐ semantic tokens provider
- ☐ signature help provider
- ☐ workspace symbol provider
- ☐ implementation of the [Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol/)
- ☐ document validation
