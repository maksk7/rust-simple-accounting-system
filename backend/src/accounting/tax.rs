use std::fmt;

pub struct Tax {
    name: String,
    percent: f32,
}

pub enum TaxCalculation {
    Exclusive,
    Inclusive,
}

impl Tax {
    pub fn new(name: String, percent: f32) -> Result<Self, Err(E)> {
        if percent > 100f32 || percent < 0f32 {
            Err(TaxPercentageOutOfRange)
        }

        Ok(Tax { name, percent })
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_percent(&mut self, percent: f32) {
        self.percent = percent;
    }
}

struct TaxPercentageOutOfRange;

impl fmt::Display for TaxPercentageOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tax percentage is out of range.")
    }
}

impl fmt::Debug for TaxPercentageOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
