# mpm

'Modern package manager'* (mpm) is a package manager written in Rust. This is
mostly and primarily a side project, for fun.

## Building mpm
`mpm` builds on stable rustc (as of 1.5). To build run `cargo build`.

[**Get `cargo/rustc`**](https://www.rust-lang.org/downloads.html)

## Contributing
Please feel free to open a PR or issue. All commits are run through rustfmt before committing with no options passed.

Get [`rust-fmt`](https://github.com/rust-lang-nursery/rustfmt)

## Current Status:
- [x] Builds a thing
	- [x] Creates a proper build environment
- [x] Packages a thing
	- [ ] Includes metadata
		- [x] Includes PKGINFO
		- [ ] Includes MTREE
	- [ ] Signed packages
- [ ] Installs a thing

## Creating a Package
`mpm` uses toml files to describe a package. The following is an example
schema (found in the `example/tar` directory), please note that this is subject to
change:
```toml
[package]
maintainers = [ "Alberto Corona<ac@albertocorona.com>" ]
desc = "Package file for testing mpm"
name = "hello-mpm"
vers = "0.0.1"
rel = "1"
arch = [""]
url = "https://github.com"
license = "BSD"
makedeps = ["make", "git"]
deps = [ ]
provides = "test"
conflicts = ["test-git"]
source = ["https://github.com/0X1A/hello-mpm/releases/download/0.0.0/hello-mpm.tar.gz"]
sha512 = ["4c62cf269ddbe5eed11d01061287dd1c2be5df36b649e5bc823d68b1437f7680d6776f9c6dc183bd7ea704fab3ce9518631f0b0e1ff397b25c77bf3f5b95e8f0"]
build = [
	'cd $src_dir',
	'make',
]
package = [
	'make DEST=$pkg_dir install'
]

[clean]
script = [
	'rm -v hello-mpm.tar.gz',
	'rm -v hello-mpm-x86_64.pkg.tar',
	'rm -rv src',
	'rm -rv pkg',
]
```

`mpm` is heavily influences by `pacman`

## Building a Package
For general help and options run `mpm -h`. To build a package run `mpm -b
PKG.toml`

## Why?
Because why not and also because why not.

<sub>*Nothing about `mpm` is particularly modern</sub>
