use super::{extended_edwards::ExtendedPoint, field::field_element::FieldElement};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct AffinePoint {
    pub x: FieldElement,
    pub y: FieldElement,
}

impl core::fmt::Debug for AffinePoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AffinePoint(****)")
    }
}

impl core::fmt::Display for AffinePoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AffinePoint(REDACTED)")
    }
}

impl AffinePoint {
    pub fn identity() -> AffinePoint {
        AffinePoint {
            x: FieldElement::zero(),
            y: FieldElement::one(),
        }
    }
    pub fn to_extended(&self) -> ExtendedPoint {
        ExtendedPoint {
            X: self.x,
            Y: self.y,
            Z: FieldElement::one(),
            T: self.x * self.y,
        }
    }
}
