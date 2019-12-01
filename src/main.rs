use std::cell::RefCell;

struct Row {
    data: String,
}

struct Query {
    equal_to: String,
}

struct NoIndex<'a> {
    rows: &'a Vec<RefCell<Row>>,
}

fn run_query(idx: NoIndex, q: Query) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for (pos, e) in idx.rows.iter().enumerate() {
        if e.borrow().data == q.equal_to {
            result.push(pos);
        }
    }
    return result;
}

fn main() {
    println!("Hello, world!");

    let mut v: Vec<RefCell<Row>> = Vec::new();

    v.push(RefCell::new(Row {
        data: String::from("foo"),
    }));
    v.push(RefCell::new(Row {
        data: String::from("bar"),
    }));

    let idx = NoIndex { rows: &v };

    let r = &(v[0]);
    println!("row {}", r.borrow().data);
    r.borrow_mut().data = String::from("quux");
    println!("row {}", r.borrow().data);

    let result = run_query(
        idx,
        Query {
            equal_to: String::from("bar"),
        },
    );
    println!("{:?}", result);
}
