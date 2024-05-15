# Diatomic Simulator

## for microwave spctrum

now: 2024 05/12
<img align="left" src="./figure/plot_spectrum_0.svg" height="600" width="1000"/>

now: 2024 05/13
<img align="left" src="./figure/plot_spectrum_1.svg" height="600" width="1000"/>

## Description

This program simulates the rotational spectrum of diatomic molecules using Rust. 
It utilizes quantum mechanical principles to calculate the energy levels and corresponding spectral lines of the molecule under consideration.

## build

- dependency
    - Ubuntu/Debian: libgtk-3-0, libgtk-3-dev
    - Arch Linux: gtk3

To build "ui_mw"

```bash
$ cargo build --bin ui_mw
```

To build [examples](./examples)

```bash
$ cargo build --example <target name> 
```
and visualize the result of exmples by jupyter lab

1. init Python environment

```bash
$ poetry install
```

2. start jupyter lab

```bash
$ poetry run jupyter lab
```
## Progress

### Current Support

- dependency    
    - Debiain/Ubuntu libgtk-3-0
    - Arch Linux gtk3

- x86_64 GNU/Linux
- WSL2 (need WSLg for GUI)

### now: 

- for maicrowave spectrum
    - logic part: Spectrum calculation based on rotational constants and other parameters.
    - GUI part: Interactive plot. 

- for electronic spectrum
    - logic part: Spectrum calculation based on rotational constants and other parameters.

## Features

- Calculates rotational energy levels
- Generates rotational spectrum
- Allows customization of molecule parameters (rotational constant, rotational temperature, band origin)

## License

This project is licensed under the GNU Lesser General Public License v3.0 - see the LICENSE file for details.
