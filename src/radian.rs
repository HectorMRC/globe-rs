//! Radian unit.

use std::ops::MulAssign;

use crate::{Float, PositiveFloat, TAU};

/// The [radian](https://en.wikipedia.org/wiki/Radian) unit, which is always a positive number within the range of [0, 2π).
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Radian(PositiveFloat);

impl From<Float> for Radian {
    fn from(value: Float) -> Self {
        if (0. ..TAU).contains(&value) {
            return Self(value.into());
        }

        let mut modulus = value % TAU;
        if value.is_sign_negative() {
            modulus = (modulus + TAU) % TAU;
        }

        Self(modulus.into())
    }
}

impl MulAssign<Float> for Radian {
    fn mul_assign(&mut self, rhs: Float) {
        *self = (self.as_float() * rhs).into();
    }
}

impl Radian {
    /// Smallest radian value.
    pub const MIN: Self = Self(PositiveFloat::MIN);
    /// Largest radian value.
    pub const MAX: Self = Self(PositiveFloat(TAU - Float::MIN_POSITIVE));

    /// Returns the value as a [`Float`].
    pub fn as_float(&self) -> Float {
        self.0.as_float()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI, TAU};

    use crate::{radian::Radian, Float};

    #[test]
    fn radiant_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: Float,
            output: Float,
        }

        vec![
            Test {
                name: "radiant within range must not change",
                input: PI,
                output: PI,
            },
            Test {
                name: "2π radiants must equals zero",
                input: TAU,
                output: 0.,
            },
            Test {
                name: "negative radiant must change",
                input: -FRAC_PI_2,
                output: TAU - FRAC_PI_2,
            },
            Test {
                name: "overflowing radiant must change",
                input: TAU + FRAC_PI_2,
                output: FRAC_PI_2,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let radiant = Radian::from(test.input).as_float();

            assert_eq!(
                radiant, test.output,
                "{}: got radiant = {}, want {}",
                test.name, radiant, test.output
            );
        });
    }
}
