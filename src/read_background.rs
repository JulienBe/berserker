use std::fs::File;
use crate::grid::*;

const BASE: u32 = 255;
const BASE_2: u32 = BASE * BASE;
const BASE_3: u32 = BASE * BASE * BASE;
const BRIDGE_COLORS: u32 =  BASE_3 * 0     + BASE_2 * 0   + BASE * 0;

pub fn read(file: File) -> Grid {
    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).unwrap();
    let w = reader.info().width;
    let h = reader.info().height;
    let mut pixels = vec![SmartPixel { types: vec![] }; (w * h) as usize];

    for y in 0..h {
        for x in 0..w {
            let buffer_index: usize = ((y * h + x) * 4) as usize;
            let pixels_index: usize = ((h - (y + 1)) * h + x) as usize;
            let t = color_to_thing(buf[buffer_index] as u32, buf[buffer_index + 1] as u32, buf[buffer_index + 2] as u32);
            pixels[pixels_index].types.push(t);
        }
    }

    let mut grid = Grid {
        w: w as usize,
        h: h as usize,
        pixels,
        bridge_top: vec![0; w as usize],
    };
    grid.update_bridge_top();
    grid
}

fn color_to_thing(r: u32, g: u32, b: u32) -> PixelType {
    match BASE_3 * r + BASE_2 * g + BASE * b {
        BRIDGE_COLORS => PixelType::Bridge,
        _ => PixelType::Nothing,
    }
}

mod tests {
    use super::*;

    #[test]
    fn load_test_image() {
        let grid = read(File::open("test_assets/background_test.png").unwrap());
        assert_eq!(grid.pixels.len(), 2 * 4);
        assert_eq!(grid.w, 4);
    }
}