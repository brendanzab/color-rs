// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


#![crate_id = "color#0.1"]
#![comment = "A library that provides types and conversions for working with
             various color formats."]
#![license = "ASL2"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(globs)]

pub use self::alpha::AlphaColor;
pub use self::channel::{Channel, FloatChannel};
pub use self::hsv::{HSV, ToHSV};
pub use self::rgb::{RGB, ToRGB};
pub use self::srgb::SRGB;
pub use self::ycbcr::YCbCr;

use std::num::{One, Zero};

pub mod alpha;
pub mod channel;
pub mod hsv;
pub mod rgb;
pub mod srgb;
pub mod ycbcr;

pub fn zero<T:Zero>() -> T { Zero::zero() }
pub fn one<T:One>() -> T { One::one() }

pub trait Color<T>: Copy {
    fn clamp_s(self, lo: T, hi: T) -> Self;
    fn clamp_c(self, lo: Self, hi: Self) -> Self;
    fn inverse(self) -> Self;
    // fn mix(&self, other: &Self, value: T) -> Self;
    // fn saturation(&self, value: T) -> Self;
    // fn exposure(&self, value: T) -> Self;
    // fn brightness(&self, value: T) -> Self;
}

pub trait FloatColor<T>: Color<T> {
    fn normalize(self) -> Self;
}
