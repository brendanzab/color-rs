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

use {Color, Color3, Color4, Channel};
use {Rgb, Hsv, Srgb, YCbCr};

#[derive(Clone, Copy, PartialEq, Eq, Show)]
pub struct AlphaColor<T, C> { pub c: C, pub a: T }

pub type Rgba<T> = AlphaColor<T, Rgb<T>>;
pub type Hsva<T> = AlphaColor<T, Hsv<T>>;
pub type Srgba<T> = AlphaColor<T, Srgb<T>>;
pub type YCbCra<T> = AlphaColor<T, YCbCr<T>>;

impl<T: Channel, C: Color<T>> Color<T> for AlphaColor<T, C> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> AlphaColor<T, C> {
        AlphaColor {
            c: self.c.clamp_s(lo, hi),
            a: self.a.clamp(lo, hi),
        }
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: AlphaColor<T, C>, hi: AlphaColor<T, C>) -> AlphaColor<T, C> {
        AlphaColor {
            c: self.c.clamp_c(lo.c, hi.c),
            a: self.a.clamp(lo.a, hi.a),
        }
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> AlphaColor<T, C> {
        AlphaColor {
            c: self.c.inverse(),
            a: self.a.invert_channel(),
        }
    }
}

impl<T: Channel, C: Color3<T>> Color4<T> for AlphaColor<T, C> {
    fn into_fixed(self) -> [T; 4] {
        match self {
            AlphaColor { c, a } => match c.into_fixed() {
                [r, g, b] => [r, g, b, a],
            },
        }
    }
}
