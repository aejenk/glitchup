pub mod void;
pub mod chaos;
pub mod loops;
pub mod reverse;
pub mod shift;
pub mod shuffle;
pub mod swap;
pub mod increase;
pub mod gradient;
pub mod multiply;
pub mod compress;

#[macro_export]
macro_rules! get_setting {
    ($category:tt, $value:tt, $config: ident, $self: ident) => {
        if let Some(range) = $config.get_option_as_single($category, $value) {
            $self.ranges.it_range = (range, range);
        } else if let Some(range) = $config.get_option_as_range($category, $value) {
            $self.ranges.it_range = range;
        } else {
            panic!("'{}' is required to be set, either globally, or under {}", $value, $category);
        }
    };
}

#[macro_export]
macro_rules! impl_configure {
    ($category:tt, [ $( $value:tt ),* ], [ $( $rangename:ident ),* ]) => {
        fn configure(&mut self, config: &Configuration) {
        $(
            if let Some(range) = config.get_option_as_single_int($category, $value) {
                self.ranges.$rangename = (range as usize, range as usize + 1);
            } else if let Some(range) = config.get_option_as_range_int($category, $value) {
                self.ranges.$rangename = (range.0 as usize, range.1 as usize);
            } else {
                panic!("'{}' is required to be set, either globally, or under {}", $value, $category);
            }
        )*
        }
    };

    ($category:tt, [ $( $value:tt ),* | $( $fvalue:tt ),* ], [ $( $rangename:ident ),*| $( $frangename:ident ),* ]) => {
        fn configure(&mut self, config: &Configuration) {
        $(
            if let Some(range) = config.get_option_as_single_int($category, $value) {
                self.ranges.$rangename = (range as usize, range as usize + 1);
            } else if let Some(range) = config.get_option_as_range_int($category, $value) {
                self.ranges.$rangename = (range.0 as usize, range.1 as usize);
            } else {
                panic!("'{}' is required to be set, either globally, or under {}", $value, $category);
            }
        )*

        $(
            if let Some(range) = config.get_option_as_single_float($category, $fvalue) {
                self.ranges.$frangename = (range as f64, range as f64 + 10e-10_f64);
            } else if let Some(range) = config.get_option_as_range_float($category, $fvalue) {
                self.ranges.$frangename = (range.0 as f64, range.1 as f64);
            } else {
                panic!("'{}' is required to be set, either globally, or under {}", $fvalue, $category);
            }
        )*

        }
    }
}

#[macro_export]
macro_rules! rangeinit {
    ($self:ident, $rng:ident, [ $( $rangename:ident => $settingname:ident ),* ]) => {
        $( $self.$settingname = $rng.gen_range($self.ranges.$rangename.0, $self.ranges.$rangename.1); )*
    };
}

pub fn index_boundary(data: &[u8]) -> (usize, usize) {
    (data.len()/50, data.len())
}