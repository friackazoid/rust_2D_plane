mod point;

use point::Point;
use std::thread;
use std::time::Duration;

fn main() {

    let mut point = Point::new(0.0, 0.0);
    point.set_target(3.0, -2.0);


    let handle = thread::spawn(move || {
        while point.update (0.1) {
            let (x, y) = point.get_current_position();
            println!("x: {}, y: {}", x, y);
            thread::sleep(Duration::from_millis(100));
        }
    });


    handle.join().unwrap();
}
