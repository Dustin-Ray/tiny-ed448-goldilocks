#![warn(clippy::just_underscores_and_digits)]
/// Module for all EC operations.
pub mod curve {
    pub mod affine;
    pub mod extended_edwards;
    pub mod projective_niels;
    pub mod twisted_edwards;
    pub mod field {
        pub mod field_element;
        pub mod lookup_table;
        pub mod scalar;
    }
}
