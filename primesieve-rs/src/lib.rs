// lib.rs
// Copyright 2016 Alexander Altman
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

use std::{mem, slice};

pub extern crate primesieve_sys as raw;

extern crate num_traits;
use num_traits::cast::cast as num_cast;
use num_traits::clamp;

extern crate odds;
use odds::debug_assert_unreachable;

pub mod max_stop {
    #[inline]
    pub fn get() -> u64 {
        unsafe { super::raw::primesieve_get_max_stop() }
    }
}

pub mod sieve_size {
    use super::{debug_assert_unreachable, num_cast};

    pub fn set<N: Into<u16>>(sieve_size: N) -> bool {
        if let Some(n_) = num_cast::<u16, i32>(sieve_size.into()) {
            if (1..=2048).contains(&n_) {
                unsafe {
                    super::raw::primesieve_set_sieve_size(n_);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    #[inline]
    pub fn get() -> u16 {
        num_cast::<i32, u16>(unsafe { super::raw::primesieve_get_sieve_size() })
            .unwrap_or_else(|| unsafe { debug_assert_unreachable() })
    }
}

pub mod num_threads {
    use super::{debug_assert_unreachable, num_cast};

    pub fn set<N: Into<Option<u64>>>(num_threads: N) -> bool {
        if let Some(n) = num_threads.into() {
            if let Some(n_) = num_cast::<u64, i32>(n) {
                if n_ >= 1 {
                    unsafe {
                        super::raw::primesieve_set_num_threads(n_);
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            unsafe {
                super::raw::primesieve_set_num_threads(-1);
            }
            true
        }
    }

    #[inline]
    pub fn get() -> u64 {
        num_cast::<i32, u64>(unsafe { super::raw::primesieve_get_num_threads() })
            .unwrap_or_else(|| unsafe { debug_assert_unreachable() })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Tupling {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl Default for Tupling {
    #[inline]
    fn default() -> Self {
        Tupling::One
    }
}

macro_rules! from_tupling_impl {
    ($t:ty) => {
        impl ::std::convert::From<$crate::Tupling> for $t {
            fn from(v: $crate::Tupling) -> $t {
                v as $t
            }
        }
    };
}

from_tupling_impl!(u8);
from_tupling_impl!(i8);
from_tupling_impl!(u16);
from_tupling_impl!(i16);
from_tupling_impl!(u32);
from_tupling_impl!(i32);
from_tupling_impl!(u64);
from_tupling_impl!(i64);
from_tupling_impl!(usize);
from_tupling_impl!(isize);

pub trait ToTupling {
    fn to_tupling(self) -> Option<Tupling>;
}

impl ToTupling for Tupling {
    #[inline]
    fn to_tupling(self) -> Option<Tupling> {
        Some(self)
    }
}

macro_rules! to_tupling_impl {
    ($t:ty) => {
        impl $crate::ToTupling for $t {
            #[inline]
            fn to_tupling(self) -> ::std::option::Option<$crate::Tupling> {
                if self == 1 {
                    ::std::option::Option::Some($crate::Tupling::One)
                } else if self == 2 {
                    ::std::option::Option::Some($crate::Tupling::Two)
                } else if self == 3 {
                    ::std::option::Option::Some($crate::Tupling::Three)
                } else if self == 4 {
                    ::std::option::Option::Some($crate::Tupling::Four)
                } else if self == 5 {
                    ::std::option::Option::Some($crate::Tupling::Five)
                } else if self == 6 {
                    ::std::option::Option::Some($crate::Tupling::Six)
                } else {
                    ::std::option::Option::None
                }
            }
        }
    };
}

to_tupling_impl!(u8);
to_tupling_impl!(i8);
to_tupling_impl!(u16);
to_tupling_impl!(i16);
to_tupling_impl!(u32);
to_tupling_impl!(i32);
to_tupling_impl!(u64);
to_tupling_impl!(i64);
to_tupling_impl!(usize);
to_tupling_impl!(isize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Count {
    pub tupling: Tupling,
    pub start: u64,
    pub stop: u64,
}

impl Count {
    #[inline]
    pub fn new() -> Self {
        Count {
            tupling: Tupling::One,
            start: 0,
            stop: max_stop::get(),
        }
    }

    pub fn tupling<T: ToTupling>(mut self, tupling: T) -> Self {
        self.tupling = tupling.to_tupling().expect("invalid tupling");
        self
    }

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = clamp(start.into(), 0, max_stop::get());
        self
    }

    pub fn stop<N: Into<u64>>(mut self, stop: N) -> Self {
        self.stop = clamp(stop.into(), 0, max_stop::get());
        self
    }

    pub fn run(self) -> Option<u64> {
        let result = match self.tupling {
            Tupling::One => unsafe { raw::primesieve_count_primes(self.start, self.stop) },
            Tupling::Two => unsafe { raw::primesieve_count_twins(self.start, self.stop) },
            Tupling::Three => unsafe { raw::primesieve_count_triplets(self.start, self.stop) },
            Tupling::Four => unsafe { raw::primesieve_count_quadruplets(self.start, self.stop) },
            Tupling::Five => unsafe { raw::primesieve_count_quintuplets(self.start, self.stop) },
            Tupling::Six => unsafe { raw::primesieve_count_sextuplets(self.start, self.stop) },
        };
        if result != raw::PRIMESIEVE_ERROR {
            Some(result)
        } else {
            None
        }
    }
}

impl Default for Count {
    #[inline]
    fn default() -> Self {
        Count::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Nth {
    pub n: i64,
    pub start: u64,
}

impl Nth {
    #[inline]
    pub fn new() -> Self {
        Nth { n: 0, start: 0 }
    }

    pub fn after<N: Into<u64>>(mut self, n: N) -> Option<Self> {
        if let Some(n_) = num_cast::<u64, i64>(n.into()) {
            if n_ >= 0 {
                self.n = -n_;
                Some(self)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn before<N: Into<u64>>(mut self, n: N) -> Option<Self> {
        if let Some(n_) = num_cast::<u64, i64>(n.into()) {
            if n_ > 0 {
                self.n = -n_;
                Some(self)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = clamp(start.into(), 0, max_stop::get());
        self
    }

    #[inline]
    pub fn run(self) -> Option<u64> {
        let result = unsafe { raw::primesieve_nth_prime(self.n, self.start) };
        if result != raw::PRIMESIEVE_ERROR {
            Some(result)
        } else {
            None
        }
    }
}

impl Default for Nth {
    #[inline]
    fn default() -> Self {
        Nth::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Print {
    pub tupling: Tupling,
    pub start: u64,
    pub stop: u64,
}

impl Print {
    #[inline]
    pub fn new() -> Self {
        Print {
            tupling: Tupling::One,
            start: 0,
            stop: max_stop::get(),
        }
    }

    pub fn tupling<T: ToTupling>(mut self, tupling: T) -> Option<Self> {
        if let Some(t) = tupling.to_tupling() {
            self.tupling = t;
            Some(self)
        } else {
            None
        }
    }

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = clamp(start.into(), 0, max_stop::get());
        self
    }

    pub fn stop<N: Into<u64>>(mut self, stop: N) -> Self {
        self.stop = clamp(stop.into(), 0, max_stop::get());
        self
    }

    pub fn execute(self) {
        match self.tupling {
            Tupling::One => unsafe { raw::primesieve_print_primes(self.start, self.stop) },
            Tupling::Two => unsafe { raw::primesieve_print_twins(self.start, self.stop) },
            Tupling::Three => unsafe { raw::primesieve_print_triplets(self.start, self.stop) },
            Tupling::Four => unsafe { raw::primesieve_print_quadruplets(self.start, self.stop) },
            Tupling::Five => unsafe { raw::primesieve_print_quintuplets(self.start, self.stop) },
            Tupling::Six => unsafe { raw::primesieve_print_sextuplets(self.start, self.stop) },
        }
    }
}

impl Default for Print {
    #[inline]
    fn default() -> Self {
        Print::new()
    }
}

pub unsafe trait Generable: Clone {
    fn type_key() -> u32;
}

unsafe impl Generable for u16 {
    #[inline]
    fn type_key() -> u32 {
        raw::UINT16_PRIMES
    }
}

unsafe impl Generable for u32 {
    #[inline]
    fn type_key() -> u32 {
        raw::UINT32_PRIMES
    }
}

unsafe impl Generable for u64 {
    #[inline]
    fn type_key() -> u32 {
        raw::UINT64_PRIMES
    }
}

unsafe impl Generable for i16 {
    #[inline]
    fn type_key() -> u32 {
        raw::INT16_PRIMES
    }
}

unsafe impl Generable for i32 {
    #[inline]
    fn type_key() -> u32 {
        raw::INT32_PRIMES
    }
}

unsafe impl Generable for i64 {
    #[inline]
    fn type_key() -> u32 {
        raw::INT64_PRIMES
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Generate {
    pub start: u64,
    pub stop: u64,
}

impl Generate {
    #[inline]
    pub fn new() -> Self {
        Generate {
            start: 0,
            stop: max_stop::get(),
        }
    }

    pub fn start<N: Into<u64>>(mut self, start: N) -> Self {
        self.start = clamp(start.into(), 0, max_stop::get());
        self
    }

    pub fn stop<N: Into<u64>>(mut self, stop: N) -> Self {
        self.stop = clamp(stop.into(), 0, max_stop::get());
        self
    }

    pub fn run<N: Generable>(self) -> Vec<N> {
        let mut size: usize = 0;
        let raw_arr = unsafe {
            raw::primesieve_generate_primes(self.start, self.stop, &mut size, N::type_key() as i32)
        };
        let result: Vec<N> = unsafe {
            slice::from_raw_parts(
                raw_arr as *mut N,
                size,
            )
        }
        .to_owned();
        unsafe {
            raw::primesieve_free(raw_arr);
        }
        result
    }
}

impl Default for Generate {
    #[inline]
    fn default() -> Self {
        Generate::new()
    }
}

#[derive(Debug)]
pub struct Iter {
    raw_iter: Box<raw::primesieve_iterator>,
    _is_reversed: bool,
}

impl Iter {
    pub fn new() -> Self {
        let mut ri = Box::new(unsafe { mem::zeroed::<raw::primesieve_iterator>() });
        unsafe { raw::primesieve_init(ri.as_mut()) };
        Iter {
            raw_iter: ri,
            _is_reversed: false,
        }
    }
}

impl Default for Iter {
    #[inline]
    fn default() -> Self {
        Iter::new()
    }
}

impl Drop for Iter {
    fn drop(&mut self) {
        unsafe { raw::primesieve_free_iterator(self.raw_iter.as_mut()) };
    }
}
