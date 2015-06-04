rustoci
=======

**Library under development. Not ready for use.**

Rust bindings for Oracle Call Interface

You need to install `Oracle Instant Client` libraries before you can link your program with this library.

Example usage:
```
extern crate rustoci;

use rustoci::Connection;

fn main() {
    let user = "apps".to_string();
    let pass = "apps".to_string();
    let db = "ehqe".to_string();

    Connection::new(user, pass, db)
        .and_then(|c| c.new_statement("select 1 from dual".to_string()))
        .and_then(|s| s.execute())
        .ok().expect("Cannot execute");

    println!("done.");
}
```
