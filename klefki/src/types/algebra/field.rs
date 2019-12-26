use crate::constrant::{IntPrimitive, COMPLEX_PREC, SECP256K1_P};
use crate::types::algebra::traits::{
    ConstP, Field, Identity, MatMul, Not, Pow as FieldPow, SecIdentity,
};
use rug::{ops::Pow, Assign, Complex, Float, Integer};
use std::any::{Any, TypeId};
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct FiniteField {
    pub value: Complex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FiniteFieldSecp256k1 {
    pub value: Complex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FiniteFieldCyclicSecp256k1 {
    pub value: Complex,
}

impl<'a> ConstP<'a> for FiniteFieldSecp256k1 {
    const P: &'a str = SECP256K1_P;
}

impl<'a> ConstP<'a> for FiniteFieldCyclicSecp256k1 {
    const P: &'a str = SECP256K1_P;
}

impl<'a> ConstP<'a> for FiniteField {
    const P: &'a str = SECP256K1_P;
}

impl FiniteField {
    pub fn new(input: &str) -> Self {
        FiniteField {
            value: Integer::from_str_radix(input, 16).expect("Cannot parse from string")
                + Complex::new(COMPLEX_PREC),
        }
    }
}

impl FiniteFieldSecp256k1 {
    pub fn new(input: &str) -> Self {
        FiniteFieldSecp256k1 {
            value: Integer::from_str_radix(input, 16).expect("Cannot parse from string")
                + Complex::new(COMPLEX_PREC),
        }
    }
}

impl FiniteFieldCyclicSecp256k1 {
    pub fn new(input: &str) -> Self {
        FiniteFieldCyclicSecp256k1 {
            value: Integer::from_str_radix(input, 16).expect("Cannot parse from string")
                + Complex::new(COMPLEX_PREC),
        }
    }
}

impl Default for FiniteFieldSecp256k1 {
    fn default() -> Self {
        let p = FiniteFieldSecp256k1::P;
        FiniteFieldSecp256k1::new(p)
    }
}

impl Default for FiniteFieldCyclicSecp256k1 {
    fn default() -> Self {
        let p = FiniteFieldCyclicSecp256k1::P;
        FiniteFieldCyclicSecp256k1::new(p)
    }
}

impl Default for FiniteField {
    fn default() -> Self {
        let p = FiniteField::P;
        FiniteField::new(p)
    }
}

macro_rules! field_trait_implement {
    ($structName: ident) => {
        impl $structName {
            #[inline]
            fn do_mod(&self, a: &dyn Any, b: &Integer) -> Complex {
                if TypeId::of::<Complex>() == a.type_id() {
                    let a = a
                        .downcast_ref::<Complex>()
                        .expect("do_mod downcast_ref to Complex Failed")
                        .clone();
                    let (ref real, ref imag) = a.into_real_imag();
                    let real = Float::with_val(
                        COMPLEX_PREC,
                        real - Float::with_val(COMPLEX_PREC, real / b),
                    );
                    let imag = Float::with_val(
                        COMPLEX_PREC,
                        imag - Float::with_val(COMPLEX_PREC, imag / b),
                    );
                    Complex::with_val(COMPLEX_PREC, (real, imag))
                } else if TypeId::of::<Integer>() == a.type_id() {
                    let a = a
                        .downcast_ref::<Integer>()
                        .expect("do_mod downcast_ref to Integer Failed");
                    Complex::new(COMPLEX_PREC) + Integer::from(a % b)
                } else {
                    unreachable!();
                }
            }
        }

        impl Identity for $structName {
            #[inline]
            fn identity() -> Self {
                $structName {
                    value: Integer::from(0)
                        % Integer::from_str_radix($structName::P, 16)
                            .expect("Cannot parse from string")
                        + Complex::new(COMPLEX_PREC),
                }
            }
        }

        impl SecIdentity for $structName {
            #[inline]
            fn sec_identity() -> Self {
                $structName {
                    value: Integer::from(1)
                        % Integer::from_str_radix($structName::P, 16)
                            .expect("Cannot parse from string")
                        + Complex::new(COMPLEX_PREC),
                }
            }
        }

        impl Field for $structName {
            #[inline]
            fn inverse(&self) -> Self {
                $structName {
                    value: Integer::from_str_radix($structName::P, 16)
                        .expect("Cannot parse from string")
                        - self.value.clone()
                        + Complex::new(COMPLEX_PREC),
                }
            }

            #[inline]
            fn sec_inverse(&self) -> Self {
                let base1 = Integer::from(1) + Complex::new(COMPLEX_PREC);
                let temp = self.value.clone();
                let (ref a, ref b) = base1.into_real_imag();
                let (ref c, ref d) = temp.into_real_imag();
                let real = (Float::with_val(COMPLEX_PREC, a * c)
                    + Float::with_val(COMPLEX_PREC, b * d))
                    / (Float::with_val(COMPLEX_PREC, c.pow(2))
                        + Float::with_val(COMPLEX_PREC, d.pow(2)));
                let imag = (Float::with_val(COMPLEX_PREC, b * c)
                    - Float::with_val(COMPLEX_PREC, a * d))
                    / (Float::with_val(COMPLEX_PREC, c.pow(2))
                        + Float::with_val(COMPLEX_PREC, d.pow(2)));
                $structName {
                    value: Complex::with_val(COMPLEX_PREC, (real, imag)),
                }
            }

            #[inline]
            fn op(&self, g: &dyn Any) -> Self {
                let ng: $structName = if g.type_id() == TypeId::of::<IntPrimitive>() {
                    let g = g.downcast_ref::<IntPrimitive>().expect("Parse Error");
                    let c = Complex::new(COMPLEX_PREC) + g.to_integer();
                    $structName { value: c }
                } else if g.type_id() == TypeId::of::<Complex>() {
                    let g = g.downcast_ref::<Complex>().expect("Parse Error");
                    $structName { value: g.clone() }
                } else {
                    unreachable!();
                };
                let a = self.value.clone() + ng.value;
                let b =
                    Integer::from_str_radix($structName::P, 16).expect("Cannot parse from string");
                let v: Complex = self.do_mod(&a, &b);
                $structName { value: v }
            }

            #[inline]
            fn sec_op(&self, g: &dyn Any) -> Self {
                let ng: $structName = if g.type_id() == TypeId::of::<IntPrimitive>() {
                    let g = g.downcast_ref::<IntPrimitive>().expect("Parse Error");
                    let c = Complex::new(COMPLEX_PREC) + g.to_integer();
                    $structName { value: c }
                } else if g.type_id() == TypeId::of::<Complex>() {
                    let g = g.downcast_ref::<Complex>().expect("Parse Error");
                    $structName { value: g.clone() }
                } else {
                    unreachable!();
                };
                let a = self.value.clone() * ng.value;
                let b =
                    Integer::from_str_radix($structName::P, 16).expect("Cannot parse from string");
                let v: Complex = self.do_mod(&a, &b);
                $structName { value: v }
            }
        }

        impl Add for $structName {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                self.op(&other)
            }
        }

        impl Sub for $structName {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                let other = other.inverse();
                self.op(&other)
            }
        }

        impl Neg for $structName {
            type Output = Self;
            fn neg(self) -> Self {
                self.inverse()
            }
        }

        impl Mul for $structName {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                self.sec_op(&other)
            }
        }

        impl Div for $structName {
            type Output = Self;
            fn div(self, other: Self) -> Self::Output {
                let other = other.sec_inverse();
                self.sec_op(&other)
            }
        }

        impl FieldPow for $structName {
            fn pow(&self, rhs: $structName) -> Self {
                let (real, _) = rhs.value.into_real_imag();
                let (identity, _) = $structName::identity().value.into_real_imag();
                let times = match real.to_integer() {
                    Some(i) => i,
                    None => unreachable!(),
                };
                let init = match identity.to_integer() {
                    Some(i) => i,
                    None => unreachable!(),
                };

                $structName {
                    value: Integer::from(init * times) + Complex::new(COMPLEX_PREC),
                }
            }
        }

        impl MatMul for $structName {
            fn mat_mul(&self, rhs: $structName) -> Self {
                let (real, _) = rhs.value.into_real_imag();
                let (identity, _) = $structName::identity().value.into_real_imag();
                let times = match real.to_integer() {
                    Some(i) => i,
                    None => unreachable!(),
                };
                let init = match identity.to_integer() {
                    Some(i) => i,
                    None => unreachable!(),
                };

                $structName {
                    value: Integer::from(init * times) + Complex::new(COMPLEX_PREC),
                }
            }
        }

        impl Not for $structName {
            fn not(&self) -> Self {
                $structName {
                    value: self.value.clone().neg(),
                }
            }
        }
    };
}

field_trait_implement!(FiniteFieldSecp256k1);
field_trait_implement!(FiniteFieldCyclicSecp256k1);
field_trait_implement!(FiniteField);
