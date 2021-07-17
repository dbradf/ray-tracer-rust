use crate::utils::equal_f64;

const MAX_COLOR: usize = 255;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn ppm_value(&self) -> String {
        format!(
            "{} {} {}",
            Self::value(self.red),
            Self::value(self.green),
            Self::value(self.blue)
        )
    }

    fn value(f: f64) -> usize {
        (MAX_COLOR as f64 * f).clamp(0.0, 255.0) as usize
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal_f64(self.red, other.red)
            && equal_f64(self.green, other.green)
            && equal_f64(self.blue, other.blue)
    }
}
impl Eq for Color {}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: (0..width * height)
                .into_iter()
                .map(|_| Color::new(0.0, 0.0, 0.0))
                .collect(),
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        let index = self.index(x, y);
        &self.pixels[index]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            let index = self.index(x, y);
            self.pixels[index] = *color;
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn to_ppm(&self) -> String {
        format!(
            "P3\n{} {}\n{}\n{}\n",
            self.width,
            self.height,
            MAX_COLOR,
            self.ppm_pixel_content()
        )
    }

    fn ppm_pixel_content(&self) -> String {
        let pixel_rows: Vec<String> = (0..self.height)
            .into_iter()
            .map(|j| self.ppm_pixel_row(j))
            .collect();
        pixel_rows.join("\n")
    }

    fn ppm_pixel_row(&self, row: usize) -> String {
        let pixel_colors: Vec<String> = (0..self.width)
            .into_iter()
            .map(|i| self.pixel_at(i, row).ppm_value())
            .collect();

        let line = pixel_colors.join(" ");
        if line.len() > 70 {
            let mut strings = vec![];
            let mut s = String::from("");
            line.split(' ').for_each(|c| {
                if s.len() + c.len() > 70 {
                    strings.push(s.clone().trim().to_string());
                    s = format!("{}", c);
                } else {
                    s = format!("{} {}", s, c);
                }
            });
            if s.len() > 0 {
                strings.push(s.trim().to_string());
            }
            strings.join("\n")
        } else {
            line
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_are_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert!(equal_f64(c.red, -0.5));
        assert!(equal_f64(c.green, 0.4));
        assert!(equal_f64(c.blue, 1.7));
    }

    #[test]
    fn test_adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn test_ppm_color_value() {
        let c1 = Color::new(1.0, 0.5, 0.0);
        let c2 = Color::new(1.5, -1.5, 0.0);

        assert_eq!(c1.ppm_value(), "255 127 0");
        assert_eq!(c2.ppm_value(), "255 0 0");
    }

    #[test]
    fn test_creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for y in 0..20 {
            for x in 0..10 {
                assert_eq!(c.pixel_at(x, y), &Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, &red);

        assert_eq!(c.pixel_at(2, 3), &red);
    }

    #[test]
    fn test_contructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert!(ppm.starts_with("P3\n5 3\n255\n"));
    }

    #[test]
    fn test_contructing_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, &c1);
        c.write_pixel(2, 1, &c2);
        c.write_pixel(4, 2, &c3);
        let ppm = c.to_ppm();

        ppm.lines().enumerate().for_each(|(i, line)| match i {
            3 => assert_eq!(line, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"),
            4 => assert_eq!(line, "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0"),
            5 => assert_eq!(line, "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"),
            _ => (),
        });

        assert_eq!(ppm.lines().count(), 3 + 3);
    }

    #[test]
    fn test_splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);

        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, &color);
            }
        }

        let ppm = c.to_ppm();
        ppm.lines().enumerate().for_each(|(i, line)| match i {
            3 => assert_eq!(
                line,
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
            ),
            4 => assert_eq!(line, "153 255 204 153 255 204 153 255 204 153 255 204 153"),
            5 => assert_eq!(
                line,
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
            ),
            6 => assert_eq!(line, "153 255 204 153 255 204 153 255 204 153 255 204 153"),
            _ => (),
        });

        assert_eq!(ppm.lines().count(), 3 + 2 * 2);
    }

    #[test]
    fn test_ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert_eq!(ppm.chars().last(), Some('\n'));
    }
}
