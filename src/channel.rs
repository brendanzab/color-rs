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

//! Color channel conversions and utility methods

use num::{Float, NumCast, Num, zero, one};
use std::{u8, u16};

pub trait Channel: Copy + Sized + Clone + PartialOrd<Self> + Num + NumCast {
    fn from<T:Channel>(chan: T) -> Self;
    fn to_channel<T:Channel>(self) -> T { Channel::from(self) }
    fn to_channel_u8(self)  -> u8;
    fn to_channel_u16(self) -> u16;
    fn to_channel_f32(self) -> f32;
    fn to_channel_f64(self) -> f64;

    fn invert_channel(self) -> Self;

    fn clamp(self, lo: Self, hi: Self) -> Self {
        if self < lo {
            lo
        } else if self > hi {
            hi
        } else {
            self
        }
    }
    
    #[inline]
    fn normalized_mul(self, rhs: Self) -> Self {
        Channel::from(self.to_channel_f32() * rhs.to_channel_f32())
    }
    
    #[inline]
    fn normalized_div(self, rhs: Self) -> Self {
        Channel::from(self.to_channel_f32() / rhs.to_channel_f32())
    }
    
    fn max() -> Self;
    
    #[inline]
    fn mix(self, rhs: Self, value: Self) -> Self {
        (self + (rhs - self).normalized_mul(value))
    }
}

impl Channel for u8 {
    #[inline] fn from<T:Channel>(chan: T) -> u8 { chan.to_channel_u8() }
    #[inline] fn to_channel_u8(self)  -> u8  { self }
    #[inline] fn to_channel_u16(self) -> u16 { ((self as u16) << 8) | self as u16 }
    #[inline] fn to_channel_f32(self) -> f32 { (self as f32) / (0xFF_u8 as f32) }
    #[inline] fn to_channel_f64(self) -> f64 { (self as f64) / (0xFF_u8 as f64) }

    #[inline] fn invert_channel(self) -> u8 { !self }
    
    #[inline] fn max() -> u8{ u8::MAX }
}

impl Channel for u16 {
    #[inline] fn from<T:Channel>(chan: T) -> u16 { chan.to_channel_u16() }
    #[inline] fn to_channel_u8(self)  -> u8  { (self >> 8) as u8 }
    #[inline] fn to_channel_u16(self) -> u16 { self }
    #[inline] fn to_channel_f32(self) -> f32 { (self / 0xFFFF) as f32 }
    #[inline] fn to_channel_f64(self) -> f64 { (self / 0xFFFF) as f64 }

    #[inline] fn invert_channel(self) -> u16 { !self }
    
    #[inline] fn max() -> u16{ u16::MAX }
}

impl Channel for f32 {
    #[inline] fn from<T:Channel>(chan: T) -> f32 { chan.to_channel_f32() }
    #[inline] fn to_channel_u8(self)  -> u8  { (self * (0xFF_u8 as f32)) as u8 }
    #[inline] fn to_channel_u16(self) -> u16 { (self * (0xFFFF_u16 as f32)) as u16 }
    #[inline] fn to_channel_f32(self) -> f32 { self }
    #[inline] fn to_channel_f64(self) -> f64 { self as f64 }

    #[inline] fn invert_channel(self) -> f32 { 1.0 - self }
    
    #[inline]
    fn normalized_mul(self, rhs: Self) -> Self {
        self * rhs
    }
    
    #[inline]
    fn normalized_div(self, rhs: Self) -> Self {
        self / rhs
    }
    
    #[inline] fn max() -> f32{ 1.0 }
}

impl Channel for f64 {
    #[inline] fn from<T:Channel>(chan: T) -> f64 { chan.to_channel_f64() }
    #[inline] fn to_channel_u8(self)  -> u8  { (self * (0xFF_u8 as f64)) as u8 }
    #[inline] fn to_channel_u16(self) -> u16 { (self * (0xFFFF_u16 as f64)) as u16 }
    #[inline] fn to_channel_f32(self) -> f32 { self as f32 }
    #[inline] fn to_channel_f64(self) -> f64 { self }

    #[inline] fn invert_channel(self) -> f64 { 1.0 - self }
    
    #[inline]
    fn normalized_mul(self, rhs: Self) -> Self {
        self * rhs
    }
    
    #[inline]
    fn normalized_div(self, rhs: Self) -> Self {
        self / rhs
    }
    
    #[inline] fn max() -> f64{ 1.0 }
}

pub trait FloatChannel: Float + Channel {
    #[inline]
    fn saturate(self) -> Self {
        Channel::clamp(self, zero(), one())
    }
}

impl FloatChannel for f32 {}
impl FloatChannel for f64 {}

#[cfg(test)]
mod tests {
    use super::Channel;

    #[test]
    fn test_to_channel_u8() {
        assert_eq!(0x00_u8.to_channel_u8(), 0x00_u8);
        assert_eq!(0x30_u8.to_channel_u8(), 0x30_u8);
        assert_eq!(0x66_u8.to_channel_u8(), 0x66_u8);
        assert_eq!(0xA0_u8.to_channel_u8(), 0xA0_u8);
        assert_eq!(0xFF_u8.to_channel_u8(), 0xFF_u8);

        assert_eq!(0x00_u8.to_channel_u16(), 0x0000_u16);
        assert_eq!(0x30_u8.to_channel_u16(), 0x3030_u16);
        assert_eq!(0x66_u8.to_channel_u16(), 0x6666_u16);
        assert_eq!(0xA0_u8.to_channel_u16(), 0xA0A0_u16);
        assert_eq!(0xFF_u8.to_channel_u16(), 0xFFFF_u16);

        assert_eq!(0x00_u8.to_channel_f32(), 0f32);
        assert_eq!(0xFF_u8.to_channel_f32(), 1f32);

        assert_eq!(0x00_u8.to_channel_f64(), 0f64);
        assert_eq!(0xFF_u8.to_channel_f64(), 1f64);
    }

    #[test]
    fn test_invert_channel_u8() {
        assert_eq!(0x00_u8.invert_channel(), 0xFF_u8);
        assert_eq!(0x66_u8.invert_channel(), 0x99_u8);
        assert_eq!(0xFF_u8.invert_channel(), 0x00_u8);
    }

    #[test]
    fn test_to_channel_u16() {
        assert_eq!(0x0000_u16.to_channel_u8(), 0x00_u8);
        assert_eq!(0x3300_u16.to_channel_u8(), 0x33_u8);
        assert_eq!(0x6666_u16.to_channel_u8(), 0x66_u8);
        assert_eq!(0xAA00_u16.to_channel_u8(), 0xAA_u8);
        assert_eq!(0xFFFF_u16.to_channel_u8(), 0xFF_u8);

        assert_eq!(0x0000_u16.to_channel_u16(), 0x0000_u16);
        assert_eq!(0x3300_u16.to_channel_u16(), 0x3300_u16);
        assert_eq!(0x6666_u16.to_channel_u16(), 0x6666_u16);
        assert_eq!(0xAA00_u16.to_channel_u16(), 0xAA00_u16);
        assert_eq!(0xFFFF_u16.to_channel_u16(), 0xFFFF_u16);

        assert_eq!(0x0000_u16.to_channel_f32(), 0f32);
        assert_eq!(0xFFFF_u16.to_channel_f32(), 1f32);

        assert_eq!(0x0000_u16.to_channel_f64(), 0f64);
        assert_eq!(0xFFFF_u16.to_channel_f64(), 1f64);
    }

    #[test]
    fn test_invert_channel_u16() {
        assert_eq!(0x0000_u16.invert_channel(), 0xFFFF_u16);
        assert_eq!(0x6666_u16.invert_channel(), 0x9999_u16);
        assert_eq!(0xFFFF_u16.invert_channel(), 0x0000_u16);
    }

    #[test]
    fn test_to_channel_f32() {
        assert_eq!(0.00f32.to_channel_u8(), 0x00);
        assert_eq!(0.25f32.to_channel_u8(), 0x3F);
        assert_eq!(0.50f32.to_channel_u8(), 0x7F);
        assert_eq!(0.75f32.to_channel_u8(), 0xBF);
        assert_eq!(1.00f32.to_channel_u8(), 0xFF);

        assert_eq!(0.00f32.to_channel_u16(), 0x0000);
        assert_eq!(0.25f32.to_channel_u16(), 0x3FFF);
        assert_eq!(0.50f32.to_channel_u16(), 0x7FFF);
        assert_eq!(0.75f32.to_channel_u16(), 0xBFFF);
        assert_eq!(1.00f32.to_channel_u16(), 0xFFFF);

        assert_eq!(0.00f32.to_channel_f32(), 0.00f32);
        assert_eq!(1.00f32.to_channel_f32(), 1.00f32);

        assert_eq!(0.00f32.to_channel_f64(), 0.00f64);
        assert_eq!(1.00f32.to_channel_f64(), 1.00f64);
    }

    #[test]
    fn test_invert_channel_f32() {
        assert_eq!(0.00f32.invert_channel(), 1.00f32);
        assert_eq!(0.50f32.invert_channel(), 0.50f32);
        assert_eq!(1.00f32.invert_channel(), 0.00f32);
    }

    #[test]
    fn test_to_channel_f64() {
        assert_eq!(0.00f64.to_channel_u8(), 0x00);
        assert_eq!(0.25f64.to_channel_u8(), 0x3F);
        assert_eq!(0.50f64.to_channel_u8(), 0x7F);
        assert_eq!(0.75f64.to_channel_u8(), 0xBF);
        assert_eq!(1.00f64.to_channel_u8(), 0xFF);

        assert_eq!(0.00f64.to_channel_u16(), 0x0000);
        assert_eq!(0.25f64.to_channel_u16(), 0x3FFF);
        assert_eq!(0.50f64.to_channel_u16(), 0x7FFF);
        assert_eq!(0.75f64.to_channel_u16(), 0xBFFF);
        assert_eq!(1.00f64.to_channel_u16(), 0xFFFF);

        assert_eq!(0.00f64.to_channel_f32(), 0.00f32);
        assert_eq!(1.00f64.to_channel_f32(), 1.00f32);

        assert_eq!(0.00f64.to_channel_f64(), 0.00f64);
        assert_eq!(1.00f64.to_channel_f64(), 1.00f64);
    }

    #[test]
    fn test_invert_channel_f64() {
        assert_eq!(0.00f64.invert_channel(), 1.00f64);
        assert_eq!(0.50f64.invert_channel(), 0.50f64);
        assert_eq!(1.00f64.invert_channel(), 0.00f64);
    }
}
