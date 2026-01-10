pub mod route;

use route::{Map, Stretch};

fn main() {
    let mut map = Map::with_capacity(15);

    map.push(Stretch {
        speed: 10,
        length: 12,
        cars: 5,
        cap: 8,
    });
    map.push(Stretch {
        speed: 10,
        length: 12,
        cars: 5,
        cap: 8,
    });
    map.push(Stretch {
        speed: 10,
        length: 12,
        cars: 5,
        cap: 8,
    });
    map.push(Stretch {
        speed: 10,
        length: 12,
        cars: 5,
        cap: 8,
    });
    map.push(Stretch {
        speed: 10,
        length: 12,
        cars: 5,
        cap: 8,
    });

    map.connect(0, 1);
    map.connect(1, 2);
    map.connect(4, 3);
    map.connect(3, 2);

    let r = map.solve(0, 4);

    println!("{:#?}", r);
}
