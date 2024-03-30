mod point;

use point::Point;

fn main() {

    let mut point = Point::new(0.0, 0.0);
    point.set_target(3.0, -2.0);


    for _ in 0..10 {
        let (x, y) = point.get_current_position();
        println!("x: {}, y: {}", x, y);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

}
