//! TupleHash per NIST [SP 800-185].
//!
//! [SP 800-185]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(any(test, doctest, feature = "std")), no_std)]

mod hash;

pub use generic_array;
pub use hash::*;
