# Line-counter
### program to count lines of code in a project

![Build Status](https://github.com/leonovk/line-counter/actions/workflows/ci.yml/badge.svg)

## Installation

Line-counter currently supports Intel Macs, M1 ARM Macs, and Linux. The tool has been tested on these platforms and is expected to work on other Unix-like systems as well. If you encounter any issues running line-counter on your system, please let me know by creating an issue on the GitHub repository.


### Unix (MacOs/Linux) manual install

This instruction works for both Linux and macOS.

Download the latest release from the [releases page](https://github.com/leonovk/line-counter/releases) for your platform. For example, if you are using an Intel Mac, download the `line-counter-x86_64-apple-darwin.tar.gz` file. For an M1 Mac, download the `line-counter-aarch64-apple-darwin.tar.gz` file.

Extract bin file from the archive:
  
```bash
tar -xzvf line-counter-aarch64-apple-darwin.tar
```

- Move the `line-counter` binary to `/usr/local/bin` if you use **mac**
- Move the `line-counter` binary to `/usr/bin` if you use **linux**
  
```bash
sudo mv line-counter /usr/bin
```
> sudo is required to move the binary to `/usr/bin`.

## Usage

```bash
line-counter -p test
```

Example output:
```
RESULT
txt: 2
total: 2
```
For more detailed information, use the help command:

```bash
line-counter --help
```

## Contributing

Contributions to Line-counter are welcome! If you have a feature request or find a bug, please create an issue on the GitHub repository. Pull requests are also welcome.
