use std::iter::Peekable;
use std::vec::IntoIter;

pub type PeekableIter<T> = Peekable<IntoIter<T>>;
