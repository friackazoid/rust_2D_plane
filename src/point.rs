// point.rs


pub struct Point {
    position: (f64, f64),
    target: (f64, f64),

}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            position: (x, y),
            target: (x, y),
        }
    }

    pub fn set_target(&mut self, target_x: f64, target_y: f64) {
        self.target = (target_x, target_y);


    }

    pub fn update(&mut self, dt: f64) -> bool {
        let (x, y) = self.position;
        let (target_x, target_y) = self.target;

        let dx = target_x - x;
        let dy = target_y - y;
        let distance = (dx * dx + dy * dy).sqrt();
        if distance > 0.1 {
            let nx = dx / distance;
            let ny = dy / distance;
            self.position = (x + nx * dt, y + ny * dt);
            return true;
        }
        return false;
    }

    pub fn get_current_position(&self) -> (f64, f64) {
        self.position
    }

}
