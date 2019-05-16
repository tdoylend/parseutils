#[cfg(test)]
mod tests;

use std::str::Chars;

pub type Matcher = Fn(char) -> bool;

pub struct Stream<'a> {
    name: &'a str,
    data: Chars<'a>,
}
impl<'a> Stream<'a> {
    pub fn new(name: &'a str, data: Chars<'a>) -> Stream<'a> {
        Stream {
            name,
            data
        }
    }
}
