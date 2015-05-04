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

use std::ops::{Mul, Div, Add, Sub, Index, IndexMut};
use std::slice;
use num::Saturating;
use {Color, Channel, FloatChannel};
use {Rgb, Rg, ToRgb, Hsv, Srgb, YCbCr};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    
    #[inline]
    fn mix(self, other: Self, value: T) -> Self {
        AlphaColor {
            c: self.c.mix(other.c, value),
            a: self.a.mix(other.a, value)
        }
    }
}

#[macro_export]
macro_rules! rgba{
    ( $r: expr, $g: expr, $b: expr, $a: expr ) => ({
        use $crate::{Rgba,Rgb};
        Rgba{ c: Rgb{ r: $r, g: $g, b: $b }, a: $a } 
    });
    ( $to_rgb: expr, $a: expr ) => ({
        use $crate::{Rgba,ToRgb};
        Rgba{ c: $to_rgb.to_rgb(), a: $a }
    });
}

impl<T:Channel> Rgba<T> {
    #[inline]
    pub fn rg(&self) -> Rg<T> {
        self.c.rg()
    }
    
    #[inline]
    pub fn rb(&self) -> Rg<T> {
        self.c.rb()
    }
    
    #[inline]
    pub fn gr(&self) -> Rg<T> {
        self.c.gr()
    }
    
    #[inline]
    pub fn gb(&self) -> Rg<T> {
        self.c.gb()
    }
    
    #[inline]
    pub fn br(&self) -> Rg<T> {
        self.c.br()
    }
    
    #[inline]
    pub fn bg(&self) -> Rg<T> {
        self.c.bg()
    }
    
    #[inline]
    pub fn ar(&self) -> Rg<T> {
        Rg{r: self.a, g: self.c.r}
    }
    
    #[inline]
    pub fn ag(&self) -> Rg<T> {
        Rg{r: self.a, g: self.c.g}
    }
    
    #[inline]
    pub fn ab(&self) -> Rg<T> {
        Rg{r: self.a, g: self.c.b}
    }
    
    #[inline]
    pub fn ra(&self) -> Rg<T> {
        Rg{r: self.c.r, g: self.a}
    }
    
    #[inline]
    pub fn ga(&self) -> Rg<T> {
        Rg{r: self.c.g, g: self.a}
    }
    
    #[inline]
    pub fn ba(&self) -> Rg<T> {
        Rg{r: self.c.b, g: self.a}
    }
    
    #[inline]
    pub fn rgb(&self) -> Rgb<T> {
        self.c.rgb()
    }
    
    #[inline]
    pub fn rbg(&self) -> Rgb<T> {
        self.c.rbg()
    }
    
    #[inline]
    pub fn bgr(&self) -> Rgb<T> {
        self.c.bgr()
    }
    
    #[inline]
    pub fn brg(&self) -> Rgb<T> {
        self.c.brg()
    }
    
    #[inline]
    pub fn grb(&self) -> Rgb<T> {
        self.c.grb()
    }
    
    #[inline]
    pub fn gbr(&self) -> Rgb<T> {
        self.c.gbr()
    }
    
    #[inline]
    pub fn rga(&self) -> Rgb<T> {
        rgb!(self.c.r,self.c.g,self.a)
    }
    
    #[inline]
    pub fn rba(&self) -> Rgb<T> {
        rgb!(self.c.r,self.c.b,self.a)
    }
    
    #[inline]
    pub fn bra(&self) -> Rgb<T> {
        rgb!(self.c.b,self.c.r,self.a)
    }
    
    #[inline]
    pub fn bga(&self) -> Rgb<T> {
        rgb!(self.c.b,self.c.g,self.a)
    }
    
    #[inline]
    pub fn gra(&self) -> Rgb<T> {
        rgb!(self.c.g,self.c.r,self.a)
    }
    
    #[inline]
    pub fn gba(&self) -> Rgb<T> {
        rgb!(self.c.g,self.c.b,self.a)
    }
    
    #[inline]
    pub fn arg(&self) -> Rgb<T> {
        rgb!(self.a,self.c.r,self.c.g)
    }
    
    #[inline]
    pub fn arb(&self) -> Rgb<T> {
        rgb!(self.a,self.c.r,self.c.b)
    }
    
    #[inline]
    pub fn agr(&self) -> Rgb<T> {
        rgb!(self.a,self.c.g,self.c.r)
    }
    
    #[inline]
    pub fn agb(&self) -> Rgb<T> {
        rgb!(self.a,self.c.g,self.c.b)
    }
    
    #[inline]
    pub fn abr(&self) -> Rgb<T> {
        rgb!(self.a,self.c.b,self.c.r)
    }
    
    #[inline]
    pub fn abg(&self) -> Rgb<T> {
        rgb!(self.a,self.c.b,self.c.g)
    }
    
    #[inline]
    pub fn rag(&self) -> Rgb<T> {
        rgb!(self.c.r,self.a,self.c.g)
    }
    
    #[inline]
    pub fn rab(&self) -> Rgb<T> {
        rgb!(self.c.r,self.a,self.c.b)
    }
    
    #[inline]
    pub fn gar(&self) -> Rgb<T> {
        rgb!(self.c.g,self.a,self.c.r)
    }
    
    #[inline]
    pub fn gab(&self) -> Rgb<T> {
        rgb!(self.c.g,self.a,self.c.b)
    }
    
    #[inline]
    pub fn bar(&self) -> Rgb<T> {
        rgb!(self.c.b,self.a,self.c.r)
    }
    
    #[inline]
    pub fn bag(&self) -> Rgb<T> {
        rgb!(self.c.b,self.a,self.c.g)
    }
    
    #[inline]
    pub fn rgba(&self) -> Rgba<T> {
        rgba!(self.c, self.a)
    }
    
    #[inline]
    pub fn rbga(&self) -> Rgba<T> {
        rgba!(self.c.r, self.c.b, self.c.g, self.a)
    }
    
    #[inline]
    pub fn grba(&self) -> Rgba<T> {
        rgba!(self.c.g, self.c.r, self.c.b, self.a)
    }
    
    #[inline]
    pub fn gbra(&self) -> Rgba<T> {
        rgba!(self.c.g, self.c.b, self.c.r, self.a)
    }
    
    #[inline]
    pub fn brga(&self) -> Rgba<T> {
        rgba!(self.c.b, self.c.r, self.c.g, self.a)
    }
    
    #[inline]
    pub fn bgra(&self) -> Rgba<T> {
        rgba!(self.c.b, self.c.g, self.c.r, self.a)
    }
    
    #[inline]
    pub fn argb(&self) -> Rgba<T> {
        rgba!(self.a, self.c.r, self.c.g, self.c.b)
    }
    
    #[inline]
    pub fn arbg(&self) -> Rgba<T> {
        rgba!(self.a, self.c.r, self.c.b, self.c.g)
    }
    
    #[inline]
    pub fn agrb(&self) -> Rgba<T> {
        rgba!(self.a, self.c.g, self.c.r, self.c.b)
    }
    
    #[inline]
    pub fn agbr(&self) -> Rgba<T> {
        rgba!(self.a, self.c.g, self.c.b, self.c.r)
    }
    
    #[inline]
    pub fn abrg(&self) -> Rgba<T> {
        rgba!(self.a, self.c.b, self.c.r, self.c.g)
    }
    
    #[inline]
    pub fn abgr(&self) -> Rgba<T> {
        rgba!(self.a, self.c.b, self.c.g, self.c.r)
    }
    
    #[inline]
    pub fn ragb(&self) -> Rgba<T> {
        rgba!(self.c.r, self.a, self.c.g, self.c.b)
    }
    
    #[inline]
    pub fn rabg(&self) -> Rgba<T> {
        rgba!(self.c.r, self.a, self.c.b, self.c.g)
    }
    
    #[inline]
    pub fn garb(&self) -> Rgba<T> {
        rgba!(self.c.g, self.a, self.c.r, self.c.b)
    }
    
    #[inline]
    pub fn gabr(&self) -> Rgba<T> {
        rgba!(self.c.g, self.a, self.c.b, self.c.r)
    }
    
    #[inline]
    pub fn barg(&self) -> Rgba<T> {
        rgba!(self.c.b, self.a, self.c.r, self.c.g)
    }
    
    #[inline]
    pub fn bagr(&self) -> Rgba<T> {
        rgba!(self.c.b, self.a, self.c.g, self.c.r)
    }
    
    #[inline]
    pub fn rgab(&self) -> Rgba<T> {
        rgba!(self.c.r, self.c.g, self.a, self.c.b)
    }
    
    #[inline]
    pub fn rbag(&self) -> Rgba<T> {
        rgba!(self.c.r, self.c.b, self.a, self.c.g)
    }
    
    #[inline]
    pub fn grab(&self) -> Rgba<T> {
        rgba!(self.c.g, self.c.r, self.a, self.c.b)
    }
    
    #[inline]
    pub fn gbar(&self) -> Rgba<T> {
        rgba!(self.c.g, self.c.b, self.a, self.c.r)
    }
    
    #[inline]
    pub fn brag(&self) -> Rgba<T> {
        rgba!(self.c.b, self.c.r, self.a, self.c.g)
    }
    
    #[inline]
    pub fn bgar(&self) -> Rgba<T> {
        rgba!(self.c.b, self.c.g, self.a, self.c.r)
    }
}


pub trait ToRgba{
    fn to_rgba<T: Channel>(&self) -> Rgba<T>;
}

impl<T: Channel, C: ToRgb> ToRgba for AlphaColor<T,C>{
    #[inline]
    fn to_rgba<U: Channel>(&self) -> Rgba<U>{
        Rgba{c: self.c.to_rgb(), a: self.a.to_channel()}
    }
}

impl<T: Channel> ToRgba for Rgb<T> {
    #[inline]
    fn to_rgba<U: Channel>(&self) -> Rgba<U>{
        Rgba{c: self.to_rgb(), a: 1.0f32.to_channel()}
    }
}

impl<T:Clone + FloatChannel> ToRgba for Hsv<T> {
    #[inline]
    fn to_rgba<U: Channel>(&self) -> Rgba<U>{
        Rgba{c: self.to_rgb(), a: 1.0f32.to_channel()}
    }
}

impl<T:Channel, C: Mul<Output=C>> Mul for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn mul(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.mul(rhs.c),
             a: self.a.normalized_mul(rhs.a) }
    }
}

impl<T:Channel + Mul<T,Output=T>, C: Mul<T,Output=C>> Mul<T> for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn mul(self, rhs: T) -> AlphaColor<T,C> {
        let color = self.c * rhs;
        AlphaColor{ c: color,
             a: self.a * rhs }
    }
}

impl<T:Channel, C: Div<Output=C>> Div for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn div(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.div(rhs.c),
             a: self.a.normalized_div(rhs.a) }
    }
}

impl<T:Channel + Div<T,Output=T>, C: Div<T,Output=C>> Div<T> for AlphaColor<T,C> {
    type Output = AlphaColor<T,C>;

    #[inline]
    fn div(self, rhs: T) -> AlphaColor<T,C> {
        let color = self.c / rhs;
        AlphaColor{ c: color,
             a: self.a / rhs }
    }
}

impl<T:Channel + Add<T,Output=T>, C: Add<Output=C>> Add for AlphaColor<T,C>{
    type Output = AlphaColor<T,C>;

    #[inline]
    fn add(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c + rhs.c,
             a: self.a + rhs.a }
    }
}

impl<T:Channel + Sub<T,Output=T>, C: Sub<Output=C>> Sub for AlphaColor<T,C>{
    type Output = AlphaColor<T,C>;

    #[inline]
    fn sub(self, rhs: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c - rhs.c,
             a: self.a - rhs.a }
    }
}

impl<T:Channel + Saturating, C: Saturating> Saturating for AlphaColor<T,C>{
    fn saturating_add(self, v: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.saturating_add(v.c),
              a: self.a.saturating_add(v.a) }
    }
    
    fn saturating_sub(self, v: AlphaColor<T,C>) -> AlphaColor<T,C> {
        AlphaColor{ c: self.c.saturating_sub(v.c),
              a: self.a.saturating_sub(v.a) }
    }
}

impl<T, C: AsRef<[T]>> Index<usize> for AlphaColor<T,C> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        self.as_ref().index(index)
    }
}

impl<T, C: AsRef<[T]> + AsMut<[T]>> IndexMut<usize> for AlphaColor<T,C> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        self.as_mut().index_mut(index)
    }
}

impl<T, C: AsRef<[T]>> AsRef<[T]> for AlphaColor<T,C> {
    fn as_ref(&self) -> &[T] {
        unsafe{ slice::from_raw_parts(&self.c.as_ref()[0], 4) }
    }
}

impl<T, C: AsMut<[T]>> AsMut<[T]> for AlphaColor<T,C> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe{ slice::from_raw_parts_mut(&mut self.c.as_mut()[0], 4) }
    }
}
