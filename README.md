# Overview

Small utility for changing the DASH Randomizer HUD colors in an assembled ROM.

# Building

Run `cargo build --release`. The binary `dhpu` will be in `target/release`. Pre-compiled builds for
Windows, Mac, and Linux can be found in the releases section.

# Usage

This command line utility can change the hard-coded HUD colors in an assembled DASH Randomizer base
ROM or a ROM produced by the randomizer. By default, it expects a ROM named "dash_working.sfc" and
a YAML file named "colors.yaml" in the working directory and outputs "dash_dhpu.sfc". You can pass
optional `--rom, -r`, `--yaml, -y` and `--output -o` parameters to change these defaults.

A template colors.yaml file is included. The top level keys have documentation comments describing
where the color is used and the values are a hex-notation 255, 255, 255 RBG value. Make sure to include
the leading "0x" when saving the file.
