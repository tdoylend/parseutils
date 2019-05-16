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
