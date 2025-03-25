# Hello, WASM! | README.md

**v0.0.1**: "hello_wasm"

*An Intro to Web Assembly Project*

---
## Getting started:

*If needed, install Rust*

`curl https://sh.rustup.rs -sSf | sh`

*You may also need to install wasm-pack*

`cargo install wasm-pack`

### 1. Fork the repo:

`gh repo fork https://github.com/cursebreakers/hello-wasm.git --clone`

### 2. Compile with `wasm-pack build --target web`

This creates the pkg folder.

### 3. Serve the project root dir and open the webpage to see your wasm script run

You can use node server for this.
- install with `npm install -g http-server`
- Serve project folder with `http-server`

Your script should run on the page in the browser when opened.

### 4. Editing src/lib.rs

Edit the WASM program in lib.rs. 

Recompile with `wasm-pack build --target web` and re-serve with `http-server` to load your changes *(you may need to clear the cache often, or disable caching altogether)*.

---
## Credits & Acknowledgements

### Author:

Esau @ [Cursebreakers LLC](https://cursebreakers.net)

### Built with:

Crates used:
| Crate     | Version |
|-----------|---------|
| wasm-bindgen | 0.2  |
| web-sys      | 0.3  |

Honorable mention to [cargo-mommy](https://github.com/Gankra/cargo-mommy) for making programming with Rust much more fun.

### License

This project is to be released under either MIT, Apache 2.0 or both.

---

Contributions are welcome! Feel free to [submit issues, pull requests, or suggestions](mailto:hello@cursebreakers.net) for improvement.


