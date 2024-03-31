// point.rs

use std::sync::{Arc, RwLock};

pub struct Point {
    position: Arc<RwLock<(f32, f32)>>,
    target: (f32, f32),

    update_thread: Option<std::thread::JoinHandle<()>>,
}

impl Drop for Point {
    fn drop(&mut self) {
        if let Some(handle) = self.update_thread.take() {
            handle.join().unwrap();
        }
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            position: Arc::new(RwLock::new( (x, y) )),
            target: (x, y),

            update_thread: None,
        }
    }

    pub fn set_target(&mut self, target_x: f32, target_y: f32) {
        
        if let Some(handle) = self.update_thread.take() {
            handle.join().unwrap();
        }

        self.target = (target_x, target_y);

        let position_clone = self.position.clone();
        let target = self.target;


        self.update_thread = Some( std::thread::spawn( move || {
            let (target_x, target_y) = target;
            let dt = 0.1;

            loop {
                let mut curr_position = position_clone.write().unwrap();
                let dx = target_x - curr_position.0;
                let dy = target_y - curr_position.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance < 0.1 {
                    break;
                }

                let nx = curr_position.0 + dx / distance * dt;
                let ny = curr_position.1 + dy / distance * dt;
                
                *curr_position = (nx, ny);
                dbg!(curr_position.0, curr_position.1);

                std::thread::sleep(std::time::Duration::from_millis(10));

            }
        }));
    }

    /*
    fn update(&mut self, dt: f64) -> bool {
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
    */

    pub fn get_current_position(&self) -> (f32, f32) {
        *self.position.read().unwrap()
    }

}
