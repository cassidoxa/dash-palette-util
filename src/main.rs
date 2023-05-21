use std::{fs::File, io::prelude::*};

use clap::Parser;
use serde::Deserialize;

mod cli;

static HDMA_TABLE_ADDR: u64 = 0x55EA;

fn main() {
    let config: cli::UtilConfig = cli::UtilCli::parse().into_config();
    let yaml_file = std::fs::read_to_string(config.yaml).expect("Failed reading YAML file.");
    let colors: ColorsYaml = serde_yaml::from_str(&yaml_file).expect("Failed parsing YAML file");
    let mut rom_bytes = std::fs::read(config.rom).expect("Failed reading ROM file.");
    let mut cur = std::io::Cursor::new(&mut rom_bytes);

    let mut hdma_table = HdmaTable::new();
    hdma_table.insert_color(0, rgb_to_snes(colors.color_one));
    hdma_table.insert_color(1, rgb_to_snes(colors.color_two));
    hdma_table.insert_color(2, rgb_to_snes(colors.color_three));
    hdma_table.insert_color(4, rgb_to_snes(colors.color_four));
    hdma_table.insert_color(5, rgb_to_snes(colors.color_five));
    hdma_table.insert_color(6, rgb_to_snes(colors.color_six));
    hdma_table.insert_color(9, rgb_to_snes(colors.color_seven));
    hdma_table.insert_color(12, rgb_to_snes(colors.color_eight));

    let hdma_bytes = hdma_table.to_le_bytes();

    cur.set_position(HDMA_TABLE_ADDR);
    cur.write(&hdma_bytes)
        .expect("Error writing HDMA table to buffer");

    let mut output = File::create(config.out).expect("Error creating output file");

    output.write(&rom_bytes).expect("Error writing output file");
}

#[derive(Debug, Copy, Clone, Default)]
struct HdmaRow {
    scanline: u8,
    color_index: u16,
    color_value: u16,
}

impl HdmaRow {
    const fn new(scanline: u8, color_index: u16, color_value: u16) -> Self {
        HdmaRow {
            scanline,
            color_index,
            color_value,
        }
    }

    const fn to_le_bytes(&self) -> [u8; 5] {
        let i = self.color_index.to_le_bytes();
        let v = self.color_value.to_le_bytes();

        [self.scanline, i[1], i[0], v[1], v[0]]
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct HdmaTable([HdmaRow; 17]);

impl HdmaTable {
    const fn new() -> Self {
        HdmaTable([
            HdmaRow::new(01, 0x0101, 0x03DD),
            HdmaRow::new(01, 0x0505, 0x700F),
            HdmaRow::new(01, 0x0606, 0x7FFF),
            HdmaRow::new(01, 0x0707, 0x0000),
            HdmaRow::new(01, 0x1919, 0x3687),
            HdmaRow::new(01, 0x1A1A, 0x7FFF),
            HdmaRow::new(07, 0x1E1E, 0x7FFF),
            HdmaRow::new(01, 0x0F0F, 0x0000),
            HdmaRow::new(01, 0x0101, 0x02DF),
            HdmaRow::new(06, 0x0505, 0x1C3B),
            HdmaRow::new(01, 0x0606, 0x2D08),
            HdmaRow::new(01, 0x0505, 0x41AD),
            HdmaRow::new(05, 0x1919, 0x71C7),
            HdmaRow::new(01, 0x0707, 0x1863),
            HdmaRow::new(01, 0x1E1E, 0x001F),
            HdmaRow::new(01, 0x1919, 0x5AD6),
            HdmaRow::new(01, 0x1A1A, 0x4A52),
        ])
    }

    fn insert_color(&mut self, idx: usize, color: u16) {
        self.0[idx].color_value = color;
    }

    fn to_le_bytes(&self) -> [u8; 85] {
        self.0
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }
}

#[derive(Debug, Copy, Clone, Default, Deserialize)]
struct ColorsYaml {
    color_one: u32,
    color_two: u32,
    color_three: u32,
    color_four: u32,
    color_five: u32,
    color_six: u32,
    color_seven: u32,
    color_eight: u32,
}

fn rgb_to_snes(v: u32) -> u16 {
    let r: u16 = (((v & 0xFF0000) >> 16) as u16) / 8;
    let g: u16 = (((v & 0x00FF00) >> 8) as u16) / 8;
    let b: u16 = ((v & 0x0000FF) as u16) / 8;

    let snes_color = r + (g << 5) + (b << 10);
    snes_color
}
