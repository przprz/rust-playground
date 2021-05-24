mod functions;
mod fn_once;
mod threads;
mod q;
mod traits;

use traits::Magnify;
use traits::Mountain;


fn main() {
    // functions::f();
    // threads::t();
    // q::q();

    // let s = String::from("ssss");
    // Magnify::magnify(&s);
    // s.magnify();

    // let m = Mountain::new();
    // m.magnify();

    fn_once::a_closure();

}
