use crate::ray::Ray;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g:f32, b: f32, _a:f32) -> Self {
        Self { r, g, b, a : _a}
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r as f32 * rhs,
            g: self.g as f32 * rhs,
            b: self.b as f32 * rhs,
            a: self.a as f32 * rhs,
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output {
            r: rhs.r as f32 * self,
            g: rhs.g as f32 * self,
            b: rhs.b as f32 * self,
            a: rhs.a as f32 * self,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }    
    }
}

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    let a = 0.5*(unit_direction.y + 1.0);
    return (1.0-a)*Color::new(1.0, 1.0, 1.0, 0.0) + a * Color::new(0.5, 0.7, 1.0, 0.0);
}

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}", pixel_color.r as i32, pixel_color.g as i32, pixel_color.b as i32);
}