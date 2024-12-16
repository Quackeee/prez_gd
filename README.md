# Project for JiMPG

Project structure based on [wasm-bindgen WebGL example](https://rustwasm.github.io/docs/wasm-bindgen/examples/webgl.html).

## Stack

This project uses:

- Node JS
- Rust to WebAssembly
- WebGL

## Setup

### Prerequisites:

1. Install [Git for Windows](https://git-scm.com/downloads/win).
2. Install [Node 22](https://nodejs.org/en/download/package-manager) either through nvm, your preferred package manager or from a binary. NVM is the recommended method - on Windows you can get it using the [nvm-windows](https://github.com/coreybutler/nvm-windows) project.
3. Install [Rust through Rustup](https://www.rust-lang.org/tools/install).
4. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer). This handles compiling to wasm, cross-language binding and packaging for linking from javascript code.

### Setup:

1. Go into the project location.
2. Run `npm install` to install node dependencies.
3. Run `npm run serve` to start the development server.
4. Open your browser and go to `http://localhost:8080`.

### Debugging in VS Code [Optional, Recommended]

1. Install [TypeScript and WebPack problem matcher](https://marketplace.visualstudio.com/items?itemName=amodio.tsl-problem-matcher) for Visual Studio Code. This will allow you to see TypeScript errors in the Problems tab.
2. Install [WebAssembly DWARF Debugging](https://marketplace.visualstudio.com/items?itemName=ms-vscode.wasm-dwarf-debugging) extension.
3. Open VS Code workspace. Run `Launch in Chrome (workspace)` task. This will open the project in Chrome with debugging enabled.
4. [Optional] Install [C/C++ DevTools Support (DWARF)](https://goo.gle/wasm-debugging-extension) in the opened browser profile, then close the browser and relaunch the debugging task. You should now be able to debug the project in VS Code and see sources in Chrome DevTools.

### Debugging in Chrome [Optional]

Project supports debugging in Chromium based browsers using the DWARF interface.
Steps to enable debugging:

1. Install any supported Chromium based browser. (Chrome, Edge, Brave, etc.)
2. Install [C/C++ DevTools Support (DWARF)](https://goo.gle/wasm-debugging-extension).
3. Open the npm serve webpage. Open DevTools (CTRL+SHIFT+I) and go to the `Console` tab. You should see a message like `Loaded debug symbols for http://localhost:8080/..., found 273 source file(s)`.
4. Go to the `Sources` tab and you should see a `file://` folder with all the source files. You can set breakpoints and debug the code.
