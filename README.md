<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/jbirbal-skydom/slint_custom_titlebar">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">project_title</h3>

  <p align="center">
    project_description
    <br />
    <a href="https://github.com/jbirbal-skydom/slint_custom_titlebar"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/jbirbal-skydom/slint_custom_titlebar">View Demo</a>
    ·
    <a href="https://github.com/jbirbal-skydom/slint_custom_titlebar/issues">Report Bug</a>
    ·
    <a href="https://github.com/jbirbal-skydom/slint_custom_titlebar/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

Here's a blank template to get started: To avoid retyping too much info. Do a search and replace with your text editor for the following: `jbirbal-skydom`, `slint_custom_titlebar`, `twitter_handle`, `linkedin_username`, `email_client`, `email`, `project_title`, `project_description`

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With
* [![Slint][slint-lang]][Slint-url]
* [![JABCode][jabcode-badge]][JABCode-url]
* [![Rust][rust-lang]][Rust-url]
* [![C][c-lang]][C-url]
* [![GCC][gcc-badge]][GCC-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

# Cross-Compiling Rust Projects

This README provides instructions and examples for setting up your Rust development environment to cross-compile applications for Windows (`x86_64-pc-windows-gnu`) and macOS (`x86_64-apple-darwin`) from a Linux system.

## Prerequisites

Ensure you have `cargo` and `rustc` installed. You can install them via Rust's official installation method, [rustup](https://rustup.rs/).

### Common Dependencies

- Install build essentials and cross-compiling tools:
  ```bash
  sudo apt update
  sudo apt install build-essential gcc g++ cmake
  ```

## Cross-Compiling for Windows

### Setup

1. **Install MinGW-w64:**
   ```bash
   sudo apt install mingw-w64
   ```

2. **Add the Windows GNU target:**
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

3. **Configure Cargo for the Windows target:**
   Create or edit the `.cargo/config.toml` file in your project directory or in your home directory (`~/.cargo/`) and add:
   ```toml
   [target.x86_64-pc-windows-gnu]
   linker = "x86_64-w64-mingw32-gcc"
   ```

- note: **Console vs GUI app:**
  - In windows the typical app is console so disable it you need to all a line in the `.cargo/config.toml` 

    ```rs
    [target.x86_64-pc-windows-gnu]
    rustflags = ["-C", "link-args=-Wl,--subsystem,windows"]
    ```


### Build Command

- To compile your project for Windows:
  ```bash
  cargo build --target=x86_64-pc-windows-gnu
  ```

## Cross-Compiling for macOS

### Setup

0.  ***Dependencies***
    - for osxcross and clang/clang++:
      ```sh
      sudo apt-get install autoconf cmake clang llvm-dev uuid-dev libssl-dev libbz2-dev
      ```
      for gcc:
      ```sh
      sudo apt-get install gcc g++ zlib1g-dev libmpc-dev libmpfr-dev libgmp-dev
      ```
      ```sh
      sudo apt-get install libgmp-dev libmpfr-dev libmpc-dev
      ```



    - XAR

        **Installing xar on Various Linux Distributions**

        - Clone the repository:
          ```sh
          git clone https://github.com/mozilla/xar
          cd xar
          ```
        - Install build dependencies:

          For most distributions, you will need `autoconf`, `automake`, `libtool`, and `make`. Ensure these are installed using your package manager.
        - Build:
          ```sh
          ./autogen.sh
          ./configure
          make
          sudo make install
          ```

    - clang

        **Installing xar on Various Linux Distributions**

        - Clone the repository:
          ```sh
          sudo apt install clang build-essential xz-utils libxml2-dev libssl-dev libbz2-dev zlib1g-dev
          ```
        - Install build dependencies:

          For most distributions, you will need `autoconf`, `automake`, `libtool`, and `make`. Ensure these are installed using your package manager.
        - Build:
          ```sh
          ./autogen.sh
          ./configure
          make
          sudo make install
          ```



1. **Install osxcross:**
   - Clone the osxcross repository:
     ```bash
     git clone https://github.com/tpoechtrager/osxcross.git
     cd osxcross
     ```
   - Download the macOS SDK (as per Apple’s software license agreement) and place it in the `tarballs` directory. ***(phracker SDK is known to be broken)***
      - Phacker
          ```sh
          cd ../
          git clone https://github.com/phracker/MacOSX-SDKs
          cd MacOSX-SDKs
          ```

          ```sh
          tar cJvf ../osxcross/tarballs/MacOSX10.14.sdk.tar.xz MacOSX10.14.sdk/
          ```
      - Docker
        ```sh
        wget https://s3.dockerproject.org/darwin/v2/MacOSX10.11.sdk.tar.xz
        ```
      - alex
        ```sh
        git clone https://github.com/alexey-lysiuk/macos-sdk/releases
        ```
      - joseluisq (best)
       ```sh
       git clone https://github.com/joseluisq/macosx-sdks/
       ```




   - Build osxcross:
     ```bash
     cd ../osxcross
     ```
      - Preperation for only building local ABI and ***build the required CLang library***
      ```sh
      sudo sed -i -e 's|-march=native||g' build_clang.sh build.sh wrapper/build_wrapper.sh
      sudo ./build_clang.sh
      ```

      - Building
      ```sh
      sudo SDK_VERSION=10.13 OSX_VERSION_MIN=10.12  ./build.sh
      ```

      - GCC
      ```sh
      sudo ./build_gcc.sh
      ```

        - Clean
      ```sh
        sudo ./cleanup.sh
      ```

2. **Add the macOS target to rustup:**
   ```bash
   rustup target add x86_64-apple-darwin
   ```

3. **Configure Cargo for the macOS target:**
   Edit or create the `.cargo/config.toml` file and add:
   ```toml
   [target.x86_64-apple-darwin]
   linker = "x86_64-apple-darwin15-gcc"
   # ar = "x86_64-apple-darwin15-ar"
   ```

   Replace `osxcross path` with the actual path to your osxcross installation.

4. **Add to path**

    ```sh
    sudo mkdir -p /usr/local/osx-ndk-x86
    sudo cp -r target/* /usr/local/osx-ndk-x86
    export PATH=/usr/local/osx-ndk-x86/bin:$PATH
    export PKG_CONFIG_ALLOW_CROSS=1
    export MACOSX_DEPLOYMENT_TARGET=10.14
    ```

### Build Command

- To compile your project for macOS:
  ```bash
  cargo build --target=x86_64-apple-darwin
  ```



# Multi-binary App

- You can only invoke `slint::include_modules!();` on the last `*.rs` in the build (`build.rs`). [compile multiple slint files]

`build.rs`
```rs
fn main() {
    slint_build::compile("ui/resize.slint").unwrap();
    slint_build::compile("ui/appwindow.slint").unwrap();
    slint_build::compile("ui/titlebar.slint").unwrap();
}
```
`appwindow.rs`
```rs
slint::slint!(import { AppWindow } from "./ui/AppWindow.slint";);

fn main() -> Result<(), slint::PlatformError> {
}
```
- note: 
  - you can add the macro at the end too and it will still work: 
  ```rs
  fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.run()
  }
  slint::slint!(import { AppWindow } from "./ui/AppWindow.slint";);
  ```


`titlebar.rs`
```rs
slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
}
```


## Notes

- Ensure all paths and configurations are correctly set according to your system and installed directories.
- Test your binaries in the target environments to ensure compatibility.
- Review and comply with all applicable licensing agreements when using proprietary SDKs or tools.









<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature

See the [open issues](https://github.com/jbirbal-skydom/slint_custom_titlebar/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the GPL License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@twitter_handle](https://twitter.com/twitter_handle) - email@email_client.com

Project Link: [https://github.com/jbirbal-skydom/slint_custom_titlebar](https://github.com/jbirbal-skydom/slint_custom_titlebar)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Building OSXcross for Rust]
* [OSXcross]
* [Slint]
* [Jabcode]
* [winit - solution]
* [Tomotroid]
* [draggable]
* [compile multiple slint files] 
  


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/jbirbal-skydom/slint_custom_titlebar.svg?style=for-the-badge
[contributors-url]: https://github.com/jbirbal-skydom/slint_custom_titlebar/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/jbirbal-skydom/slint_custom_titlebar.svg?style=for-the-badge
[forks-url]: https://github.com/jbirbal-skydom/slint_custom_titlebar/network/members
[stars-shield]: https://img.shields.io/github/stars/jbirbal-skydom/slint_custom_titlebar.svg?style=for-the-badge
[stars-url]: https://github.com/jbirbal-skydom/slint_custom_titlebar/stargazers
[issues-shield]: https://img.shields.io/github/issues/jbirbal-skydom/slint_custom_titlebar.svg?style=for-the-badge
[issues-url]: https://github.com/jbirbal-skydom/slint_custom_titlebar/issues
[license-shield]: https://img.shields.io/github/license/jbirbal-skydom/slint_custom_titlebar.svg?style=for-the-badge
[license-url]: https://github.com/jbirbal-skydom/slint_custom_titlebar/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/screenshot.png
[rust-lang]: https://img.shields.io/badge/Rust-f74c00?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[c-lang]: https://img.shields.io/badge/C-00599C?style=for-the-badge&logo=c&logoColor=white
[C-url]: "https://en.wikipedia.org/wiki/C_(programming_language)"
[slint-lang]: https://img.shields.io/badge/Slint-7F52FF?style=for-the-badge&logo=slint&logoColor=white
[Slint-url]: https://slint-ui.com/
[jabcode-badge]: https://img.shields.io/badge/JABCode-00eded?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB2ZXJzaW9uPSIxLjIiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDI1IDI1IiB3aWR0aD0iMjUiIGhlaWdodD0iMjUiPgoJPHRpdGxlPmphYmNvZGVfYmFkZ2U8L3RpdGxlPgoJPHN0eWxlPgoJCS5zMCB7IGZpbGw6ICNmZmZmZmYgfSAKCTwvc3R5bGU+Cgk8cGF0aCBpZD0iUGF0aCAwIiBjbGFzcz0iczAiIGQ9Im0wIDE1di0xNWgxNXY1aC0xMHYxMHoiLz4KCTxwYXRoIGlkPSJQYXRoIDAiIGNsYXNzPSJzMCIgZD0ibTYgMTV2LTloOXYzaC02djZ6Ii8+Cgk8cGF0aCBpZD0iUGF0aCAwIiBjbGFzcz0iczAiIGQ9Im0xMCAxNXYtNWg1djV6Ii8+Cgk8cGF0aCBpZD0iUGF0aCAwIiBjbGFzcz0iczAiIGQ9Im0xMCAxNmg2di02aDN2OWgtOXoiLz4KCTxwYXRoIGlkPSJQYXRoIDAiIGNsYXNzPSJzMCIgZD0ibTEwIDIwaDEwdi0xMGg1djE1aC0xNXoiLz4KPC9zdmc+
[JABCode-url]: https://jabcode.org
[gcc-badge]: https://img.shields.io/badge/GCC-4E9A06?style=for-the-badge&logo=gnu&logoColor=white
[GCC-url]: https://gcc.gnu.org/



<!-- LINKS -->
[compile multiple slint files]:(https://github.com/slint-ui/slint/issues/3217)
[Building OSXcross for Rust]:(https://github.com/etrombly/gtk_osx_cross/blob/master/README.md)
[OSXcross]:(https://github.com/tpoechtrager/osxcross)
[Slint]:(https://github.com/slint-ui/slint)
[Jabcode]:(https://jabcode.org/)
[winit - solution]:(https://github.com/slint-ui/slint/discussions/3355)
[Tomotroid]:(https://github.com/Vadoola/Tomotroid)
[draggable]:(https://github.com/slint-ui/slint/pull/2558)