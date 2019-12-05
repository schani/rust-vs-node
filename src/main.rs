use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

type Value = Rc<String>;

trait Listener {
    fn set_dirty(&mut self);
}

type WeakListener = Weak<RefCell<dyn Listener>>;

trait ValueProducer {
    fn get_current(&mut self) -> Value;
    fn add_listener(&mut self, l: WeakListener);
}

type RcValueProducer = Rc<RefCell<dyn ValueProducer>>;

struct Row {
    data: Value,
    listeners: Vec<WeakListener>,
}

impl Row {
    fn new(data: &str) -> Row {
        return Row {
            data: Rc::new(String::from(data)),
            listeners: vec![],
        };
    }

    fn set(&mut self, v: Value) {
        self.data = v;
        for l in self.listeners.iter() {
            match l.upgrade() {
                Some(rc) => rc.borrow_mut().set_dirty(),
                None => (),
            }
        }
    }
}

impl ValueProducer for Row {
    fn get_current(&mut self) -> Value {
        return Rc::clone(&self.data);
    }

    fn add_listener(&mut self, l: WeakListener) {
        self.listeners.push(l);
    }
}

struct ToLowercase {
    input: Rc<RefCell<dyn ValueProducer>>,
    current: Option<Value>,
}

impl ToLowercase {
    fn new(input: Rc<RefCell<dyn ValueProducer>>) -> ToLowercase {
        ToLowercase {
            input,
            current: None,
        }
    }
}

impl Listener for ToLowercase {
    fn set_dirty(&mut self) {
        self.current = None;
    }
}

impl ValueProducer for ToLowercase {
    fn get_current(&mut self) -> Value {
        match &self.current {
            None => {
                let result = Rc::new(String::from(
                    self.input.borrow_mut().get_current().to_lowercase(),
                ));
                self.current = Some(Rc::clone(&result));
                return result;
            }
            Some(x) => Rc::clone(&x),
        }
    }

    fn add_listener(&mut self, _l: Weak<RefCell<dyn Listener>>) {}
}

struct Query {
    equal_to: Value,
}

struct NoIndex {
    rows: Vec<RcValueProducer>,
}

impl NoIndex {
    fn new(rows: &Vec<RcValueProducer>) -> NoIndex {
        let mut idx = NoIndex { rows: vec![] };
        for r in rows.iter() {
            idx.rows.push(Rc::clone(r));
        }
        return idx;
    }

    fn run_query(&self, q: &Query) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for (pos, e) in self.rows.iter().enumerate() {
            if e.borrow_mut().get_current() == q.equal_to {
                result.push(pos);
            }
        }
        return result;
    }
}

fn make_to_lowercase(input: RcValueProducer) -> RcValueProducer {
    let tl = Rc::new(RefCell::new(ToLowercase::new(Rc::clone(&input))));
    input
        .borrow_mut()
        .add_listener(Rc::downgrade(&tl) as Weak<RefCell<dyn Listener>>);
    return tl;
}

fn main() {
    for _i in 0..10 {
        let mut v: Vec<RcValueProducer> = Vec::new();
        let r1 = Rc::new(RefCell::new(Row::new("Foo")));
        let r2 = Rc::new(RefCell::new(Row::new("BAr")));
        let tl = make_to_lowercase(Rc::clone(&r1) as RcValueProducer);
        v.push(Rc::clone(&tl) as RcValueProducer);
        v.push(Rc::clone(&r2) as RcValueProducer);
        let idx = NoIndex::new(&v);
        let result = idx.run_query(&Query {
            equal_to: Rc::new(String::from("quux")),
        });
        assert_eq!(result.len(), 0);

        let q1 = Query {
            equal_to: Rc::new(String::from("foo")),
        };
        let q2 = Query {
            equal_to: Rc::new(String::from("BAr")),
        };

        for _j in 0..1000000 {
            // r1.borrow_mut().set(Rc::new(String::from("quuX")));
            let result = idx.run_query(&q1);
            assert_eq!(result.len(), 1);

            // println!("row {}", tl.borrow_mut().get_current());
            // r1.borrow_mut().set(Rc::new(String::from("QuuX")));
            // println!("row {}", tl.borrow_mut().get_current());
            let result = idx.run_query(&q2);
            assert_eq!(result.len(), 1);
            // println!("{:?}", result);
        }
    }
}
