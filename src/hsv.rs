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

use super::{Color, FloatColor, one, zero};
use channel::{Channel, FloatChannel};
use rgb::{RGB, ToRGB};

fn cast<T: num::NumCast, U: num::NumCast>(n: T) -> U {
    num::cast(n).unwrap()
}

#[deriving(Clone, Eq)]
pub struct HSV<T> { h: T, s: T, v: T }

impl<T:FloatChannel> HSV<T> {
    pub fn new(h: T, s: T, v: T) -> HSV<T> {
        HSV { h: h, s: s, v: v }
    }
}

impl<T:FloatChannel> Color<T> for HSV<T> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(&self, lo: T, hi: T) -> HSV<T> {
        HSV::new(self.h.clamp(&lo, &hi), // Should the hue component be clamped?
                 self.s.clamp(&lo, &hi),
                 self.v.clamp(&lo, &hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(&self, lo: &HSV<T>, hi: &HSV<T>) -> HSV<T> {
        HSV::new(self.h.clamp(&lo.h, &hi.h),
                 self.s.clamp(&lo.s, &hi.s),
                 self.v.clamp(&lo.v, &hi.v))
    }

    /// Inverts the color.
    #[inline]
    fn inverse(&self) -> HSV<T> {
        HSV::new(self.h.invert_degrees(),
                 self.s.invert_channel(),
                 self.v.invert_channel())
    }
}

impl<T:FloatChannel> FloatColor<T> for HSV<T> {
    /// Normalizes the components of the color. Modulo `360` is applied to the
    /// `h` component, and `s` and `v` are clamped to the range `(0,1)`.
    #[inline]
    fn normalize(&self) -> HSV<T> {
        HSV::new(self.h.normalize_degrees(),
                 self.s.normalize_channel(),
                 self.v.normalize_channel())
    }
}

pub trait ToHSV {
    fn to_hsv<U:FloatChannel>(&self) -> HSV<U>;
}

impl ToHSV for u32 {
    #[inline]
    fn to_hsv<U:FloatChannel>(&self) -> HSV<U> {
        fail!("Not yet implemented")
    }
}

impl ToHSV for u64 {
    #[inline]
    fn to_hsv<U:FloatChannel>(&self) -> HSV<U> {
        fail!("Not yet implemented")
    }
}

impl<T:Clone + FloatChannel> ToHSV for HSV<T> {
    #[inline]
    fn to_hsv<U:FloatChannel>(&self) -> HSV<U> {
        HSV::new(self.h.to_channel(),
                 self.s.to_channel(),
                 self.v.to_channel())
    }
}

impl<T:Clone + FloatChannel> ToRGB for HSV<T> {
    fn to_rgb<U:Channel>(&self) -> RGB<U> {
        // Algorithm taken from the Wikipedia article on HSL and HSV:
        // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV

        let chr = self.v * self.s;
        let h = self.h / cast(60);

        // the 2nd largest component
        let x = chr * (one::<T>() - ((h % cast(2)) - one()).abs());

        let mut rgb =
            if      (h < cast(1)) { RGB::new(chr.clone(), x, zero()) }
            else if (h < cast(2)) { RGB::new(x, chr.clone(), zero()) }
            else if (h < cast(3)) { RGB::new(zero(), chr.clone(), x) }
            else if (h < cast(4)) { RGB::new(zero(), x, chr.clone()) }
            else if (h < cast(5)) { RGB::new(x, zero(), chr.clone()) }
            else if (h < cast(6)) { RGB::new(chr.clone(), zero(), x) }
            else                  { RGB::new(zero(), zero(), zero()) };

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
    use hsv::*;
    use rgb::*;

    #[test]
    fn test_hsv_to_hsv() {
        assert_eq!(HSV::new::<f64>(0.0, 0.0, 1.0).to_hsv::<f32>(), HSV::new::<f32>(0.0, 0.0, 1.0));
        assert_eq!(HSV::new::<f64>(0.0, 1.0, 0.6).to_hsv::<f32>(), HSV::new::<f32>(0.0, 1.0, 0.6));
        assert_eq!(HSV::new::<f64>(120.0, 1.0, 0.6).to_hsv::<f32>(), HSV::new::<f32>(120.0, 1.0, 0.6));
        assert_eq!(HSV::new::<f64>(240.0, 1.0, 0.6).to_hsv::<f32>(), HSV::new::<f32>(240.0, 1.0, 0.6));
    }

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(HSV::new::<f32>(0.0, 0.0, 1.0).to_rgb::<u8>(), RGB::new::<u8>(0xFF, 0xFF, 0xFF));
        assert_eq!(HSV::new::<f32>(0.0, 1.0, 0.6).to_rgb::<u8>(), RGB::new::<u8>(0x99, 0x00, 0x00));
        assert_eq!(HSV::new::<f32>(120.0, 1.0, 0.6).to_rgb::<u8>(), RGB::new::<u8>(0x00, 0x99, 0x00));
        assert_eq!(HSV::new::<f32>(240.0, 1.0, 0.6).to_rgb::<u8>(), RGB::new::<u8>(0x00, 0x00, 0x99));
    }
}
