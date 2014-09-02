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

use std::num;

use {Color, FloatColor, Color3, one, zero};
use {Channel, FloatChannel};
use {Rgb, ToRgb};

fn cast<T: num::NumCast, U: num::NumCast>(n: T) -> U {
    num::cast(n).unwrap()
}

#[deriving(Clone, PartialEq, Eq, Show)]
pub struct Hsv<T> { pub h: T, pub s: T, pub v: T }

impl<T: FloatChannel> Hsv<T> {
    pub fn new(h: T, s: T, v: T) -> Hsv<T> {
        Hsv { h: h, s: s, v: v }
    }
}

impl<T:FloatChannel> Color<T> for Hsv<T> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> Hsv<T> {
        Hsv::new(self.h.clamp(lo, hi), // Should the hue component be clamped?
                 self.s.clamp(lo, hi),
                 self.v.clamp(lo, hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: Hsv<T>, hi: Hsv<T>) -> Hsv<T> {
        Hsv::new(self.h.clamp(lo.h, hi.h),
                 self.s.clamp(lo.s, hi.s),
                 self.v.clamp(lo.v, hi.v))
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> Hsv<T> {
        Hsv::new(self.h.invert_degrees(),
                 self.s.invert_channel(),
                 self.v.invert_channel())
    }
}

impl<T:FloatChannel> FloatColor<T> for Hsv<T> {
    /// Normalizes the components of the color. Modulo `360` is applied to the
    /// `h` component, and `s` and `v` are clamped to the range `(0,1)`.
    #[inline]
    fn normalize(self) -> Hsv<T> {
        Hsv::new(self.h.normalize_degrees(),
                 self.s.normalize_channel(),
                 self.v.normalize_channel())
    }
}

impl<T: FloatChannel> Color3<T> for Hsv<T> {
    fn into_fixed(self) -> [T, ..3] {
        match self {
            Hsv { h, s, v } => [h, s, v],
        }
    }
}

pub trait ToHsv {
    fn to_hsv<U:FloatChannel>(&self) -> Hsv<U>;
}

impl ToHsv for u32 {
    #[inline]
    fn to_hsv<U:FloatChannel>(&self) -> Hsv<U> {
        fail!("Not yet implemented")
    }
}

impl ToHsv for u64 {
    #[inline]
    fn to_hsv<U:FloatChannel>(&self) -> Hsv<U> {
        fail!("Not yet implemented")
    }
}

impl<T:Clone + FloatChannel> ToHsv for Hsv<T> {
    #[inline]
    fn to_hsv<U:FloatChannel>(&self) -> Hsv<U> {
        Hsv::new(self.h.to_channel(),
                 self.s.to_channel(),
                 self.v.to_channel())
    }
}

impl<T:Clone + FloatChannel> ToRgb for Hsv<T> {
    fn to_rgb<U:Channel>(&self) -> Rgb<U> {
        // Algorithm taken from the Wikipedia article on HSL and Hsv:
        // http://en.wikipedia.org/wiki/HSL_and_Hsv#From_Hsv

        let chr = self.v * self.s;
        let h = self.h / cast(60u8);

        // the 2nd largest component
        let x = chr * (one::<T>() - ((h % cast(2u8)) - one()).abs());

        let mut rgb =
            if      h < cast(1u8) { Rgb::new(chr.clone(), x, zero()) }
            else if h < cast(2u8) { Rgb::new(x, chr.clone(), zero()) }
            else if h < cast(3u8) { Rgb::new(zero(), chr.clone(), x) }
            else if h < cast(4u8) { Rgb::new(zero(), x, chr.clone()) }
            else if h < cast(5u8) { Rgb::new(x, zero(), chr.clone()) }
            else if h < cast(6u8) { Rgb::new(chr.clone(), zero(), x) }
            else                  { Rgb::new(zero(), zero(), zero()) };

        // match the value by adding the same amount to each component
        let mn = self.v - chr;

        rgb.r = rgb.r + mn;
        rgb.g = rgb.g + mn;
        rgb.b = rgb.b + mn;

        rgb.to_rgb::<U>()
    }
}

#[cfg(test)]
mod tests {
    use {Hsv, ToHsv};
    use {Rgb, ToRgb};

    #[test]
    fn test_hsv_to_hsv() {
        assert_eq!(Hsv::<f64>::new(0.0, 0.0, 1.0).to_hsv::<f32>(),   Hsv::<f32>::new(0.0, 0.0, 1.0));
        assert_eq!(Hsv::<f64>::new(0.0, 1.0, 0.6).to_hsv::<f32>(),   Hsv::<f32>::new(0.0, 1.0, 0.6));
        assert_eq!(Hsv::<f64>::new(120.0, 1.0, 0.6).to_hsv::<f32>(), Hsv::<f32>::new(120.0, 1.0, 0.6));
        assert_eq!(Hsv::<f64>::new(240.0, 1.0, 0.6).to_hsv::<f32>(), Hsv::<f32>::new(240.0, 1.0, 0.6));
    }

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(Hsv::<f32>::new(0.0, 0.0, 1.0).to_rgb::<u8>(),   Rgb::<u8>::new(0xFF, 0xFF, 0xFF));
        assert_eq!(Hsv::<f32>::new(0.0, 1.0, 0.6).to_rgb::<u8>(),   Rgb::<u8>::new(0x99, 0x00, 0x00));
        assert_eq!(Hsv::<f32>::new(120.0, 1.0, 0.6).to_rgb::<u8>(), Rgb::<u8>::new(0x00, 0x99, 0x00));
        assert_eq!(Hsv::<f32>::new(240.0, 1.0, 0.6).to_rgb::<u8>(), Rgb::<u8>::new(0x00, 0x00, 0x99));
    }
}
