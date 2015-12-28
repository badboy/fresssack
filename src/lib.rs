//! Eat through your path, one string at a time.

pub struct Fresssack {
    path: String,
    size: usize,
    pos: usize,
}

impl Fresssack {
    pub fn new(path: String) -> Fresssack {
        Fresssack {
            size: path.len(),
            path: path,
            pos: 1,
        }
    }

    pub fn curr(&self) -> &str {
        &self.path[(self.pos-1)..self.size]
    }

    pub fn prev(&self) -> &str {
        &self.path[0..(self.pos-1)]
    }

    pub fn is_root(&self) -> bool {
        self.pos >= self.size
    }

    fn at(&self, index: usize) -> Option<char> {
        self.path.chars().skip(self.pos + index).next()
    }

    fn subs(&self, length: usize) -> &str {
        &self.path[self.pos..(self.pos+length)]
    }

    fn shift(&mut self, offset: usize) {
        self.pos += offset+1;
    }

    pub fn consume(&mut self, name: &str) -> bool {
        if self.is_root() {
            return false;
        }

        let len = name.len();

        match self.at(len) {
            None | Some('/') => {
                if self.subs(len) == name {
                    self.shift(len);
                    return true;
                } else {
                    return false;
                }
            },
            _ => return false,
        }
    }
}

#[test]
fn consume() {
    let mut path = Fresssack::new("/foo/bar/baz".into());

    assert_eq!(false, path.consume("bar"));
    assert_eq!("", path.prev());
    assert_eq!("/foo/bar/baz", path.curr());
    assert_eq!(false, path.is_root());

	assert_eq!(false, path.consume("fo"));
    assert_eq!("", path.prev());
    assert_eq!("/foo/bar/baz", path.curr());
    assert_eq!(false, path.is_root());

	assert_eq!(true, path.consume("foo"));
    assert_eq!("/foo", path.prev());
    assert_eq!("/bar/baz", path.curr());
    assert_eq!(false, path.is_root());

	assert_eq!(false, path.consume("foo"));
    assert_eq!("/foo", path.prev());
    assert_eq!("/bar/baz", path.curr());
    assert_eq!(false, path.is_root());

	assert_eq!(true, path.consume("bar/baz"));
    assert_eq!("/foo/bar/baz", path.prev());
    assert_eq!("", path.curr());
    assert_eq!(true, path.is_root());


	assert_eq!(false, path.consume("baz"));
    assert_eq!("/foo/bar/baz", path.prev());
    assert_eq!("", path.curr());
    assert_eq!(true, path.is_root());
}
