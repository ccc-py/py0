use std::fmt;

pub fn die(msg: &str) -> ! {
    eprintln!("error: {}", msg);
    std::process::exit(1);
}

pub fn xstrdup(s: &str) -> String {
    s.to_string()
}

#[derive(Clone)]
pub struct PtrVec<T> {
    items: Vec<T>,
}

impl<T> PtrVec<T> {
    pub fn new() -> Self {
        PtrVec { items: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        PtrVec {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.items.get(i)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.items.get_mut(i)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.items
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> Default for PtrVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StrBuf {
    buf: String,
}

impl StrBuf {
    pub fn new() -> Self {
        StrBuf { buf: String::new() }
    }

    pub fn append_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }
}

impl Default for StrBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StrBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.buf)
    }
}