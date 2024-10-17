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

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// These `fn`s are from the C header "primesieve/primesieve_iterator.h", but
// are declared as `static inline`, so we must bind to them in a roundabout way.
#[inline]
pub unsafe fn primesieve_next_prime(pi: *mut primesieve_iterator) -> u64 {
    primesieve_next_prime_auxbind(pi)
}
#[inline]
pub unsafe fn primesieve_prev_prime(pi: *mut primesieve_iterator) -> u64 {
    primesieve_prev_prime_auxbind(pi)
}
#[link(name = "primesieve_auxbind")]
extern "C" {
    fn primesieve_next_prime_auxbind(pi: *mut primesieve_iterator) -> u64;
    fn primesieve_prev_prime_auxbind(pi: *mut primesieve_iterator) -> u64;
}

pub struct PrimeIterator {
    it: primesieve_iterator,
}

impl PrimeIterator {
    pub fn new(start: u64, stop_hint: Option<u64>) -> Self {
        let mut it: primesieve_iterator = unsafe { core::mem::zeroed() };
        let stop = stop_hint.unwrap_or(unsafe { primesieve_get_max_stop() });
        unsafe {
            primesieve_init(&mut it);
            primesieve_skipto(&mut it, start, stop);
        }
        PrimeIterator { it }
    }
}

impl Drop for PrimeIterator {
    fn drop(&mut self) {
        unsafe {
            primesieve_free_iterator(&mut self.it);
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let p = unsafe { primesieve_next_prime(&mut self.it) };
        Some(p)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::mem;

    #[test]
    fn iterator_test() {
        let mut pi: primesieve_iterator = unsafe { mem::zeroed() };
        unsafe {
            primesieve_init(&mut pi);
            primesieve_skipto(&mut pi, 1, 10)
        }
        let x = unsafe { primesieve_next_prime(&mut pi) };
        unsafe {
            primesieve_skipto(&mut pi, 10, 1);
        }
        let y = unsafe { primesieve_prev_prime(&mut pi) };
        unsafe {
            primesieve_free_iterator(&mut pi);
        }
        assert_eq!(x, 2);
        assert_eq!(y, 7);
    }

    #[test]
    fn iterator_struct() {
        let mut pi = PrimeIterator::new(0, Some(10));
        assert_eq!(pi.next().unwrap(), 2);
        assert_eq!(pi.next().unwrap(), 3);
        assert_eq!(pi.next().unwrap(), 5);
        assert_eq!(pi.next().unwrap(), 7);
    }
}
