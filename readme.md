WIP

----

This project is used to enhance the security and integration capabilities of [code-server](https://github.com/coder/code-server) .

# features

## Security enhancements

### Code watermarking
1. ✅Watermark display
1. Watermark anti-cracking

### Code manipulation
1. Encrypt Cut and Paste and Copy
    1. ✅Main Panel process
    1. Console [canvas] process
    1. Developer tools process
1. Disable code download
1. Disable copy remote file url
1. Disable code sharing
1. Operation anti-cracking
1. Make a docker image with network restrictions

### Behavior monitoring
1. ✅Heartbeat detection
1. Abnormal behavior alerts

### Self-security protection
1. ✅Wrapping algorithms and data using rustwasm
1. Further protection of algorithms and data using custom VMs

## Integration enhancements

### Code submission specification
1. ✅Commit message can only select specified content
1. Specify content to integrate with Issue platform

# Use

## Out of the box

todo

## Manual compilation

1. set up the `code-server` environment and compile code according to https://github.com/coder/code-server/blob/main/docs/CONTRIBUTING.md
1. clone this project
1. copy `paths/code-server.patch` to the root of the code-server project and execute `git am code-server.patch`
1. copy `paths/vscode.patch` to the code-server project under `lib/vscode` and execute `git am vscode.patch`
1. compile code-server project according to https://github.com/coder/code-server/blob/main/docs/CONTRIBUTING.md#build

# Contributing

1. set up the `code-server` environment and compile code according to https://github.com/coder/code-server/blob/main/docs/CONTRIBUTING.md
1. set up the `rustwasm` environment according to https://rustwasm.github.io/docs/book/game-of-life/setup.html
1. clone this project, package wasm `wasm-pack build --target web`
1. copy `pkg/vscode_starsys.js` / `pkg/vscode_starsys_bg.wasm` to the code-server project under `lib/vscode/src/vs/code/browser/workbench/pkg`
1. execute `yarn watch` in the code-server project


# Q&A

### Development with code-server

When developing with code-server, you need to avoid conflicts between the project and code-server's own services. We can modify `scripts.watch` in `package.json` . Add a new port, e.g:

```json
"watch": "LOG_LEVEL=trace PORT=3000 VSCODE_DEV=1 VSCODE_IPC_HOOK_CLI= NODE_OPTIONS='--max_old_space_size=32384 --trace-warnings' ts-node . /ci/dev/watch.ts --user-data-dir=/tmp/vscode_dev"
```
