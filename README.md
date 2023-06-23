# rust-devcontainer

A sample project to bootstrap your Rust development environment inside a container.

The following tools are used:
- [Docker Compose](https://docs.docker.com/compose/)
- [VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers)

### Demo
![Running and Debugging a Rust program](doc/example.gif)

Inside each devcontainer the following VS Code extensions are installed:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- [Gitless](https://marketplace.visualstudio.com/items?itemName=maattdd.gitless)

If you don't want to use any of the above or want to add additional extensions just update the `customizations.vscode.extensions` list inside [devcontainer.json](.devcontainer/devcontainer.json#L9)
