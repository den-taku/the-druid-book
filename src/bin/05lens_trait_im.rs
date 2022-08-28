use druid::Lens;
use std::rc::Rc;

struct Container {
    inner: String,
    another: String,
}

struct InnerLens;

impl Lens<Container, String> for InnerLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &Container, f: F) -> V {
        f(&data.inner)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut Container, f: F) -> V {
        f(&mut data.inner)
    }
}

struct Container2 {
    first_name: Rc<String>,
    last_name: Rc<String>,
    arg: u16,
}

struct Name {
    first: Rc<String>,
    last: Rc<String>,
}

struct NameLens;

impl Lens<Container2, Name> for NameLens {
    fn with<V, F: FnOnce(&Name) -> V>(&self, data: &Container2, f: F) -> V {
        let first = Rc::clone(&data.first_name);
        let last = Rc::clone(&data.last_name);
        f(&Name { first, last })
    }

    fn with_mut<V, F: FnOnce(&mut Name) -> V>(&self, data: &mut Container2, f: F) -> V {
        let first = Rc::clone(&data.first_name);
        let last = Rc::clone(&data.last_name);
        let mut name = Name { first, last };
        let ret = f(&mut name);
        data.first_name = name.first;
        data.last_name = name.last;
        ret
    }
}

fn main() {}
