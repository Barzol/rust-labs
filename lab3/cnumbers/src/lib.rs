pub mod solution{
    use std::fmt::{Display, Formatter};
    use std::hash::Hash;
    use std::ops::{Add, AddAssign};
    use std::convert::From;
    
    #[derive(Debug, Copy, Clone,Default, PartialEq, PartialOrd)]
    pub struct ComplexNumber {
        real : f64,
        imag : f64,
    }

    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError{
        ImaginaryNotZero
    }

    impl ComplexNumber {
        // getter
        pub fn real(&self) -> f64 { self.real }
        pub fn imag(&self) -> f64 { self.imag }

        // new complex number
        pub fn new(real : f64, imag: f64) -> Self {
            Self{ real, imag }
        }

        // from real
        pub fn from_real(real : f64) -> Self {
            Self { real, imag: 0.0 }
        }

        pub fn to_tuple(&self) -> (f64,f64){
            (self.real, self.imag)
        }

        pub fn modulus(&self) -> f64 {
            (self.real.powi(2) + self.imag.powi(2)).sqrt()
        }

    }

    impl Display for ComplexNumber {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    impl Add for ComplexNumber {
        type Output = Self;

        // write 'self' or 'self:Self' is the same
        fn add(self, rhs: Self) -> Self {
            Self {
                real : self.real + rhs.real,
                imag : self.imag + rhs.imag,
            }
        }
    }

    impl Add<f64> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: f64) -> Self {
            Self {
                real : self.real + rhs,
                imag : self.imag
            }
        }
    }

    impl AddAssign for ComplexNumber {

        fn add_assign(&mut self, rhs: Self) {
            *self = Self {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            };

        }
    }

    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = ComplexNumber;

        // write 'self' or 'self:Self' is the same
        fn add(self, rhs: &ComplexNumber) -> Self::Output {

            Self::Output {
                real : self.real + rhs.real,
                imag : self.imag + rhs.imag,
            }
        }
    }

    impl Add<&ComplexNumber> for &ComplexNumber {
        type Output = ComplexNumber;

        // write 'self' or 'self:Self' is the same
        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            
            Self::Output {
                real : self.real + rhs.real,
                imag : self.imag + rhs.imag,
            }
        }
    }


//    impl Into<f64> for ComplexNumber {
//
//        fn into(self) -> f64 {
//            if self.imag != 0.0 {
//                panic!("mamt");
//            } else {
//                self.real
//            }
//            
//        }
//    }

    impl TryInto<f64> for ComplexNumber {
        type Error = ComplexNumberError;
    
        fn try_into(self) -> Result<f64, Self::Error> {
            if self.imag == 0.0 {
                Ok(self.real)
            } else {
                Err(ComplexNumberError::ImaginaryNotZero)
            }
        }
    }

    impl From<f64> for ComplexNumber {
    fn from(value: f64) -> Self {
        ComplexNumber {
            real: value,
            imag: 0.0,
            }
        }
    }

    impl Eq for ComplexNumber{}
    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.modulus().total_cmp(&other.modulus())
        }
    }

    impl AsRef<f64> for ComplexNumber{
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    impl AsMut<f64> for ComplexNumber{
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber{
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.real.to_bits().hash(state);
            self.imag.to_bits().hash(state);
        }
    }


}