# Line-counter

## program to count lines of code in a project

![Build Status](https://github.com/leonovk/line-counter/actions/workflows/ci.yml/badge.svg)

## Installation

Line-counter currently supports Intel Macs, M1 ARM Macs, and Linux. The tool has been tested on these platforms and is expected to work on other Unix-like systems as well. If you encounter any issues running line-counter on your system, please let me know by creating an issue on the GitHub repository.

### Unix (MacOS/Linux)

This instruction works for both Linux and macOS.

```bash
curl -fsSL https://raw.githubusercontent.com/leonovk/line-counter/master/install.sh | bash
```

You can enter the following command to verify that the installation was successful.

```bash
line-counter --version
```

Command `--help` will offer you a list of possible commands

## Usage

```bash
line-counter -p test
```

Example output:

```shel
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
