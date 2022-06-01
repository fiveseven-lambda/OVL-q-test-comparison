/*
Copyright (c) 2022 Atsushi Komaba

This file is part of OVL-test.

OVL-test is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

OVL-test is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with OVL-test.  If not, see <https://www.gnu.org/licenses/>.
*/

pub trait UnsignedInt {
    fn bit_width(self) -> u32;
}

macro_rules! impl_unsigned_int {
    ($($t:ty),*) => {
        $(impl UnsignedInt for $t {
            fn bit_width(self) -> u32 {
                Self::BITS - self.leading_zeros()
            }
        })*
    }
}

impl_unsigned_int!(u32, usize);

#[test]
fn test_bit_width() {
    assert_eq!(0u32.bit_width(), 0);
    assert_eq!(0usize.bit_width(), 0);
    assert_eq!(u32::MAX.bit_width(), 32);
    assert_eq!(usize::MAX.bit_width(), usize::BITS);
}
