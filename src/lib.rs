pub mod bit_iterator;
pub mod decoder_v2;
pub use bit_iterator::BitIterator;
use bitstream_io::{BitWriter, LittleEndian};
use std::fs::*;
use std::io::prelude::*;
use std::io::*;
use std::path::Path;
use image::*;
use std::io;
use std::borrow::BorrowMut;
pub use crate::decoder_v2::SteganoDecoderV2;

pub struct SteganoEncoder {
    target: Option<String>,
    carrier: Option<image::DynamicImage>,
    source: Option<std::fs::File>,
}

pub trait Encoder {
    fn hide(&self) -> &Self;
}

pub trait Decoder {
    fn unveil(&mut self) -> &mut Self;
}

impl Default for SteganoEncoder {
    fn default() -> Self {
        Self {
            target: None,
            carrier: None,
            source: None
        }
    }
}

impl SteganoEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn use_carrier_image(&mut self, input_file: &str) -> &mut Self {
        self.carrier = Some(
            image::open(Path::new(input_file))
                .expect("Carrier image was not readable."));
        self
    }

    pub fn write_to(&mut self, output_file: &str) -> &mut Self {
        self.target = Some(output_file.to_string());
        self
    }

    pub fn take_data_to_hide_from(&mut self, input_file: &str) -> &mut Self {
        self.source = Some(
            File::open(input_file)
                .expect("Source file was not readable."));
        self
    }
}

impl Encoder for SteganoEncoder {
    fn hide(&self) -> &Self {
        let carrier = self.carrier.as_ref().unwrap();
        let (width, height) = carrier.dimensions();
        let mut bit_iter = BitIterator::new(self.source.as_ref().unwrap());
        let mut target: RgbaImage = ImageBuffer::new(width, height);

        #[inline]
        fn bit_wave(byte: u8, bit: Option<u8>) -> u8 {
            let mut b = 0;
            match bit {
                None => {}
                Some(byt) => b = byt,
            }

            (byte & 0xFE) | b
        }

        for x in 0..width {
            for y in 0..height {
                let image::Rgba(data) = carrier.get_pixel(x, y);
                target.put_pixel(x, y,  Rgba([
                    bit_wave(data[0], bit_iter.next()),
                    bit_wave(data[1], bit_iter.next()),
                    bit_wave(data[2], bit_iter.next()),
                    data[3],
                ]));
            }
        }

        target.save(self.target.as_ref().unwrap()).unwrap();

        self
    }
}

