// Copyright 2019 Thomas Doylend
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::*;

#[test]
fn it_works() {
    assert_eq!(2+2,4);
}

#[test]
fn instantiate_stream() {
    let mut stream = Stream::new("","abc".chars());

    assert_eq!(Some('a'),stream.peek());
    assert_eq!(Some('a'),stream.peek());
    assert_eq!(Some('a'),stream.next());
    assert_eq!(Some('b'),stream.next());
    assert_eq!(Some('c'),stream.peek());
    assert_eq!(Some('c'),stream.next());
    assert_eq!(None,stream.next());
    assert_eq!(None,stream.peek());
}

#[test]
fn munch_test() {
    let mut stream = Stream::new("","a.api".chars());

    let func = char::is_alphanumeric;

    assert_eq!(Some('a'),stream.test(func));
    assert_eq!(Some('a'),stream.munch(func));
    assert_eq!(None,stream.munch(func));
    assert_eq!(None,stream.test(func));
    assert_eq!(None,stream.munch(func));
    assert_eq!(Some('.'),stream.next());
    assert_eq!(Some('a'),stream.munch(func));
    assert_eq!(Some('p'),stream.munch(func));
    assert_eq!(Some('i'),stream.munch(func));
    assert_eq!(None,stream.munch(func));
    assert_eq!(None,stream.munch(func));
    assert_eq!(None,stream.munch(func));
}

#[test]
fn munch_seq_test() {
    let mut stream = Stream::new(""," somedata someotherdata".chars());

    let alnum = char::is_alphanumeric; 
    let white = char::is_whitespace;

    assert_eq!(None, stream.munch_seq(alnum));
    assert_eq!(Some(" ".to_string()), stream.munch_seq(white));
    assert_eq!(Some("somedata".to_string()), stream.munch_seq(alnum));
    assert_eq!(None, stream.munch_seq(alnum));
    assert_eq!(Some(" ".to_string()), stream.munch_seq(white));
    assert_eq!(Some("someotherdata".to_string()), stream.munch_seq(alnum));
    assert_eq!(None, stream.munch_seq(alnum));
    assert_eq!(None, stream.munch_seq(white));
}

#[test]
fn look_test() {
    let mut stream = Stream::new("","abcdefgh".chars());

    assert_eq!(Some('a'),stream.next());
    assert_eq!(Some('b'),stream.next());
    assert_eq!(Some('c'),stream.look(0));
    assert_eq!(Some('d'),stream.look(1));
    assert_eq!(Some('g'),stream.look(4));
    assert_eq!(Some('e'),stream.look(2));
    assert_eq!(Some('c'),stream.next());
    assert_eq!(Some('d'),stream.look(0));
}

#[test]
fn test_seq_test() {
    let mut stream = Stream::new("", " somedata someotherdata".chars());

    assert_eq!(None, stream.test_seq(char::is_alphanumeric));
    assert_eq!(Some(" ".to_string()), stream.munch_seq(char::is_whitespace));
    assert_eq!(None, stream.munch_seq(char::is_whitespace));
    assert_eq!(Some("somedata".to_string()),stream.test_seq(char::is_alphanumeric));
    assert_eq!(Some("somedata".to_string()),stream.test_seq(char::is_alphanumeric));
    assert_eq!(Some("somedata".to_string()),stream.munch_seq(char::is_alphanumeric));
    assert_eq!(None, stream.test_seq(char::is_alphanumeric));
    assert_eq!(Some(" ".to_string()), stream.munch_seq(char::is_whitespace));
    assert_eq!(Some("someotherdata".to_string()), stream.test_seq(char::is_alphanumeric));
    assert_eq!(Some("someotherdata".to_string()), stream.munch_seq(char::is_alphanumeric));
    assert_eq!(None, stream.test_seq(char::is_alphanumeric));
}

#[test]
fn matchers_test() {
    let mut stream = Stream::new(""," aaaabbb".chars());

    assert_eq!(None, stream.munch_seq(matchers::char('a')));
    assert_eq!(Some(' '), stream.munch(matchers::char(' ')));
    assert_eq!(Some("aaaa".to_string()), stream.munch_seq(matchers::char('a')));
    assert_eq!(Some("bbb".to_string()), stream.munch_seq(matchers::char('b')));
    assert_eq!(None, stream.munch_seq(matchers::any));
}

#[test]
fn position_test() {
    let mut stream = Stream::new("","alpha\nbeta\r\n\nsomething".chars());

    assert_eq!(Some("alpha".to_string()),stream.munch_seq(matchers::except(matchers::line_terminator)));
    
    assert_eq!(0, stream.line);
    assert_eq!(5, stream.column);
    
    assert_eq!(Some('\n'),stream.next());
    assert_eq!(Some('b'),stream.next());

    assert_eq!(1,stream.line);
    assert_eq!(1,stream.column);

    assert_eq!(Some("eta".to_string()), stream.munch_seq(matchers::except(matchers::line_terminator)));
    assert_eq!(Some("\r\n\n".to_string()), stream.munch_seq(matchers::line_terminator));
    
    assert_eq!(3,stream.line);
    assert_eq!(0,stream.column);
}

#[test]
fn current_line_test() {
    let mut stream = Stream::new("","alpha\r\nbeta\r\n".chars());

    assert_eq!("alpha".to_string(),stream.current_line());
    assert_eq!(Some("alpha".to_string()),stream.munch_seq(matchers::except(matchers::line_terminator)));
    assert_eq!("alpha".to_string(),stream.current_line());

    assert_eq!(Some('\r'),stream.next());
    assert_eq!("beta".to_string(), stream.current_line());
    assert_eq!(Some('\n'),stream.next());
    assert_eq!("beta".to_string(), stream.current_line());

    assert_eq!(Some('b'),stream.next());
    assert_eq!("beta".to_string(), stream.current_line());

    assert_eq!(Some("eta".to_string()), stream.munch_seq(matchers::except(matchers::line_terminator)));
    assert_eq!(Some("\r\n".to_string()), stream.munch_seq(matchers::line_terminator));

    assert_eq!("".to_string(), stream.current_line());

}
