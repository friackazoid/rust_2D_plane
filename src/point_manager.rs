// point_manager.rs

use crate::point::Point;
use rand::Rng;

pub struct PointManager {
    points: Vec<Point>,
}

impl PointManager {

    pub fn new( n : usize ) -> PointManager {
        let mut points = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            points.push(Point::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)));
        }

        PointManager { points, }
    }

    pub fn print_points(&self) {
        for (i, point) in self.points.iter().enumerate() {
            let (x, y) = point.get_current_position();
            println!("{}: x: {}, y: {}",i, x, y);
        }
    }

}

