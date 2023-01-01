
# skgit

![Frame 37](https://user-images.githubusercontent.com/54967427/210161916-d070cc7e-c1ba-4ecc-8a50-3adbc9495bd4.png)
<p align="center">
<a href="https://github.com/Mr-peipei/skgit/actions/workflows/actions.yaml"><img src="https://github.com/Mr-peipei/skgit/actions/workflows/actions.yaml/badge.svg" alt="Action info"></a>
<a href="https://crates.io/crates/skgit"><img src="https://img.shields.io/crates/v/skgit.svg?colorB=319e8c" alt="Version info"></a><br>
</p>

Welcome to skgit.  
skgit is a Cli git utilities tool written by Rust.

## Supported OS

UNIX only.  
This tool not supported on Windows because of now skim's dependency library  not supporting Windows.  
If you want to use skgit in Windows, please try to install on WSL2.
Sorry for inconvenience.  


## Installation

```bash
$ cargo install skgit
```

## Usage

### Command

<!-- You can git add by interactive search by skim. -->
And in preview window, there is a diff that you selected in skim.
```bash
$ skgit add
```
![skgit add demo](https://user-images.githubusercontent.com/54967427/206841812-9216f9b8-efc5-4bda-8d75-01f26148e600.gif)



And easily checkout your local repository.
```bash
$ skgit checkout
```
![skgit checkout demo](https://user-images.githubusercontent.com/54967427/206456666-51a868be-8e73-49ea-a785-415563e5f4ac.gif)


## Release Note

- 2022/11/30 version 0.1.0
  - Implement add command
- 2022/11/30 version 0.2.0
  - Implement checkout command


## LICENSE

The MIT License (MIT)

## Contribution

All Contribution are welcome.
