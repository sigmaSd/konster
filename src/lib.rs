//! This crate exposes const equivalents of standard library types.
//!
//! ```rust
//!    use konster::kstr::GKStr;
//!
//!    const _: () = {
//!        let mut str = GKStr::<20>::new();
//!        str = str.push(4);
//!        let (str, val) = match str.pop() {
//!             Some((str,val)) => (str, val),
//!             _ => unreachable!(),
//!        };
//!        if !str.is_empty() {
//!             panic!("Str is not empty");
//!        }
//!    };
//! ```
#![warn(missing_docs)]
pub mod kmap;
/// This module contains GKStr type which is a String like struct that can be used in const context
pub mod kstr;
/// This module contains GKVec type which is a String like struct that can be used in const context
pub mod kvec;
