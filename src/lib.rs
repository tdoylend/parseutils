#[cfg(test)]
mod tests;

pub mod matchers;

use std::str::Chars;
use std::collections::VecDeque;

pub struct Stream<'a> {
    name: &'a str,
    data: Chars<'a>,
    buffer: VecDeque<char>,
    pub line: usize,
    pub column: usize,
    current_line: String,
    first_term: bool
}
impl<'a> Stream<'a> {
    pub fn new(name: &'a str, data: Chars<'a>) -> Stream<'a> {
        Stream {
            name,
            data,
            buffer: VecDeque::new(),
            line: 0,
            column: 0,
            first_term: false,
            current_line: String::new()
        }
    }

    pub fn fill(&mut self, amount: usize) {
        if self.buffer.len() < amount {
            let delta = amount - self.buffer.len();
            'fill: for _ in 0..delta {
                match self.data.next() {
                    Some(i) => self.buffer.push_back(i),
                    None => {
                        break 'fill;
                    }
                }
            }
        }
    }

    pub fn current_line(&mut self) -> String {
        let mut buffer = self.current_line.clone();

        buffer.push_str(&self.test_seq(matchers::except(matchers::line_terminator)).unwrap_or("".to_string()));
    
        buffer
    }

    pub fn peek(&mut self) -> Option<char> {
        self.fill(1);
        self.buffer.get(0).copied()
    }

    pub fn next(&mut self) -> Option<char> {
        self.fill(1);
        let c = self.buffer.pop_front();

        if c.is_some() {
            if matchers::line_terminator(c.unwrap()) {
                self.current_line = String::new();
                if c.unwrap() == '\r' {
                    self.first_term = true;
                } else if c.unwrap() == '\n' && self.first_term {
                    self.first_term = false;
                    self.line -= 1;
                }
                self.line += 1;
                self.column = 0;
            } else  {
                self.current_line.push(c.unwrap());
                self.first_term = false;
                self.column += 1;
            }
        }

        c
    }

    pub fn look(&mut self, position: usize) -> Option<char> {
        self.fill(position+1);
        self.buffer.get(position).copied()
    }

    pub fn test<T>(&mut self, matcher: T) -> Option<char>
        where T: Fn(char) -> bool
    {
        let x = self.peek();
        match x {
            Some(c) => {
                if matcher(c) {
                    Some(c)
                } else {
                    None
                }
            },
            None => None
        }
    }

    pub fn test_seq<T>(&mut self, matcher: T) -> Option<String>
        where T: Fn(char) -> bool
    {
        let mut s = String::new();
        s.push(self.test(&matcher)?);
        let mut i = 1;

        while self.look(i).is_some() && matcher(self.look(i).unwrap()) {
            s.push(self.look(i).unwrap());
            i += 1;
        }
        Some(s)
    }

    pub fn munch<T>(&mut self, matcher: T) -> Option<char> 
        where T: Fn(char) -> bool
    {
        let x = self.peek();
        match x {
            Some(c) => {
                if matcher(c) {
                    self.next()
                } else {
                    None
                }
            },
            None => None
        }
    }

    pub fn munch_seq<T>(&mut self, matcher: T) -> Option<String> 
        where T: Fn(char) -> bool
    {
        let mut s = String::new(); 
        s.push(self.munch(&matcher)?);
        
        while self.test(&matcher).is_some() {
            s.push(self.next().unwrap());
        }

        Some(s)
    }

    
}
