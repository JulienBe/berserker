#[derive(Clone, Debug, PartialEq)]
pub enum PixelType {
    Nothing = 0,
    Bridge  = 1,
}
#[derive(Clone)]
pub struct SmartPixel {
    pub(crate) types: Vec<PixelType>,
}

pub struct Grid {
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<SmartPixel>,
    pub bridge_top: Vec<usize>,
}

impl Grid {
    pub(crate) fn update_bridge_top(&mut self) {
        (0..self.w).for_each( |x| {
            let bridge_level = (0..self.h).find(|&y| self.contains(x as usize, y as usize, PixelType::Bridge));
            match bridge_level {
                None => self.bridge_top[x] = 0,
                Some(y) => self.bridge_top[x] = y,
            }
        });
    }
    pub(crate) fn contains(&self, x: usize, y: usize, t: PixelType) -> bool {
        let pixel = self.get(x, y);
        pixel.types.contains(&t)
    }
    pub(crate) fn line(&self, x: usize, w: usize, y: usize) -> &[SmartPixel] {
        let bounds = (self.convert(&x, &y), self.convert(&(x + w), &y));
        &self.pixels[bounds.0..bounds.1]
    }
    pub(crate) fn get(&self, x: usize, y: usize) -> &SmartPixel {
        &self.pixels[self.convert(&x, &y)]
    }
    fn convert(&self, x: &usize, y: &usize) -> usize {
        (y * self.w) + x
    }
}

mod tests {
    use super::*;

    #[test]
    fn get_info_from_grid() {
        let mut pixels = vec![SmartPixel { types: vec![] }; (8) as usize];
        pixels[0].types.push(PixelType::Nothing);
        pixels[1].types.push(PixelType::Bridge);
        pixels[2].types.push(PixelType::Bridge);
        pixels[3].types.push(PixelType::Nothing);

        pixels[4].types.push(PixelType::Bridge);
        pixels[5].types.push(PixelType::Nothing);
        pixels[6].types.push(PixelType::Nothing);
        pixels[7].types.push(PixelType::Bridge);
        let grid = Grid {
            w: 4,
            h: 2,
            pixels,
            bridge_top: vec![],
        };
        assert_eq!(grid.get(2, 1).types[0], PixelType::Bridge);
        assert_eq!(grid.get(2, 0).types[0], PixelType::Nothing);
        let line_0: &[SmartPixel] = grid.line(0, 4, 0);
        assert_eq!(line_0.get(0).unwrap().types.first().unwrap(), &PixelType::Bridge)
    }
}