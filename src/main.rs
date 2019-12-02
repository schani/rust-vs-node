use std::cell::RefCell;
use std::rc::Rc;

type Value = Rc<String>;

trait ValueProducer {
    fn get_current(&mut self) -> Value;
    fn add_listener(&mut self, l: Rc<RefCell<dyn Listener>>);
}

trait Listener {
    fn set_dirty(&mut self);
}

struct Row {
    data: Value,
    listeners: Vec<Rc<RefCell<dyn Listener>>>,
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
            l.borrow_mut().set_dirty()
        }
    }
}

impl ValueProducer for Row {
    fn get_current(&mut self) -> Value {
        return Rc::clone(&self.data);
    }

    fn add_listener(&mut self, l: Rc<RefCell<dyn Listener>>) {
        self.listeners.push(l);
    }
}

struct ToLowerCase {
    input: Rc<RefCell<dyn ValueProducer>>,
    current: Option<Value>,
}

impl ToLowerCase {
    fn new(input: Rc<RefCell<dyn ValueProducer>>) -> ToLowerCase {
        ToLowerCase {
            input,
            current: None,
        }
    }
}

impl Listener for ToLowerCase {
    fn set_dirty(&mut self) {
        self.current = None;
    }
}

impl ValueProducer for ToLowerCase {
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

    fn add_listener(&mut self, _l: Rc<RefCell<dyn Listener>>) {}
}

// struct Query {
//     equal_to: Value,
// }

// struct NoIndex {
//     rows: &'a Vec<RefCell<Row>>,
// }

// fn run_query(idx: NoIndex, q: Query) -> Vec<usize> {
//     let mut result: Vec<usize> = Vec::new();
//     for (pos, e) in idx.rows.iter().enumerate() {
//         if e.borrow().data == q.equal_to {
//             result.push(pos);
//         }
//     }
//     return result;
// }

fn main() {
    let mut v: Vec<Rc<RefCell<dyn ValueProducer>>> = Vec::new();

    let r1 = Rc::new(RefCell::new(Row::new("Foo")));
    let r2 = Rc::new(RefCell::new(Row::new("BAr")));
    v.push(Rc::clone(&r1) as Rc<RefCell<dyn ValueProducer>>);
    v.push(Rc::clone(&r2) as Rc<RefCell<dyn ValueProducer>>);

    // let idx = NoIndex { rows: &v };

    let tl = Rc::new(RefCell::new(ToLowerCase::new(
        Rc::clone(&r1) as Rc<RefCell<dyn ValueProducer>>
    )));
    r1.borrow_mut()
        .add_listener(Rc::clone(&tl) as Rc<RefCell<dyn Listener>>);

    println!("row {}", tl.borrow_mut().get_current());
    r1.borrow_mut().set(Rc::new(String::from("QuuX")));
    println!("row {}", tl.borrow_mut().get_current());

    // let result = run_query(
    //     idx,
    //     Query {
    //         equal_to: String::from("bar"),
    //     },
    // );
    // println!("{:?}", result);
}
