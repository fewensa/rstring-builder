use std::ffi::OsStr;

const DEFAULT_CAPACITY: usize = 392;

#[derive(Debug, PartialEq)]
pub struct StringBuilder {
  chars: Vec<char>
}

impl StringBuilder {
  pub fn new() -> StringBuilder {
    StringBuilder::with_capacity(DEFAULT_CAPACITY)
  }

  pub fn with_capacity(size: usize) -> StringBuilder {
    StringBuilder {
      chars: Vec::with_capacity(size),
    }
  }

  pub fn append<T: Vcharsable>(&mut self, buff: T) -> &mut StringBuilder {
    self.chars.append(buff.vechars().as_mut());
    self
  }

  pub fn len(&self) -> usize {
    self.chars.len()
  }

  pub fn delete_at(&mut self, start: usize) -> &mut StringBuilder {
    self.delete(start, start + 1)
  }

  pub fn delete(&mut self, start: usize, end: usize) -> &mut StringBuilder {
    if end == 0 {
      panic!("end index must be greater then 0. end: {}", end);
    }
    if end <= start {
      panic!("End index must be greater than start. start: {} end: {}", start, end);
    }
    if end > self.chars.len() {
      panic!("Out of index range. end: {}", end);
    }
    for _i in start..end {
      self.chars.remove(start);
    }
    self
  }

  pub fn clear(&mut self) -> &mut StringBuilder {
    self.chars.clear();
    self
  }

  pub fn string(&self) -> String {
    self.chars.clone().into_iter().collect()
  }
}

impl ToString for StringBuilder {
  fn to_string(&self) -> String {
    self.string()
  }
}

pub trait Vcharsable {
  fn vechars(&self) -> Vec<char>;
}

impl Vcharsable for String {
  fn vechars(&self) -> Vec<char> {
    self.chars().collect()
  }
}

impl Vcharsable for OsStr {
  fn vechars(&self) -> Vec<char> {
    self.to_str().unwrap().chars().collect()
  }
}

impl<'a> Vcharsable for &'a str {
  fn vechars(&self) -> Vec<char> {
    self.chars().collect()
  }
}

impl Vcharsable for char {
  fn vechars(&self) -> Vec<char> {
    let mut vec = Vec::with_capacity(1);
    vec.push(*self);
    vec
  }
}
