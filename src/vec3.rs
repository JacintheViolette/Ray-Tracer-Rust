#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn lenght(self) -> f32 {
        self.squared_lenght().sqrt()
    }

    pub fn squared_lenght(self) -> f32 {
        self.x * self.x * self.y * self.y * self.z * self.z
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.lenght()
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn add_vec3() {
        let mut test = Vec3::new(1.0, 1.0, 1.0);
        test += Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), Vec3::new(2.0, 2.0, 2.0) + Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(Vec3::new(5.0, 5.0, 5.0), test);
    }

    #[test]
    fn sub_vec3() {
        let mut test = Vec3::new(4.0, 4.0, 4.0);
        test -= Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), Vec3::new(4.0, 4.0, 4.0) - Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), test)
    }

    #[test]
    fn mul_vec3() {
        let mut test = Vec3::new(4.0, 4.0, 4.0);
        test *= 2.0;
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), Vec3::new(1.5, 1.5, 1.5) * Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), 1.5 * Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), Vec3::new(2.0, 2.0, 2.0) * 1.5);
        assert_eq!(Vec3::new(8.0, 8.0, 8.0), test);
    }

    #[test]
    fn div_vec3() {
        let mut test = Vec3::new(6.0, 6.0, 6.0);
        test /= 2.0;
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), Vec3::new(6.0, 6.0, 6.0) / Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), Vec3::new(6.0, 6.0, 6.0) / 2.0);
        assert_eq!(Vec3::new(2.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0), 2.0 / Vec3::new(6.0, 6.0, 6.0));
        assert_eq!(Vec3::new(3.0, 3.0, 3.0), test);
    }
}
