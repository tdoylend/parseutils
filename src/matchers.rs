// Copyright 2019 Thomas Doylend
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn char(k: char) -> impl Fn(char) -> bool {
    move |x: char| {
        x == k
    }
}

pub fn any(_: char) -> bool {
    true
}

pub fn except<T>(f: T) -> impl Fn(char) -> bool
    where T: Fn(char) -> bool
{
    move |x: char| {
        !f(x)
    }
}

pub fn line_terminator(x: char) -> bool {
    (x == '\n') | (x == '\r')
}
