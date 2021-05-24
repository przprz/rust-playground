mod p;

#[derive(Debug, Copy, Clone)]
enum T {
    First,
    Second,
}

struct C {
    of_type: T,
    description: String,
}

pub fn q() {
    let t = C {
        of_type: T::First,
        description: "some C".to_string()
    };
    let tt = use_c(&t);
    let _p=p::P;
    std::thread::spawn(move ||{ println!("tt: {}", tt)});
}

fn use_c(c: &C) -> String {
    c.description.clone()
}

