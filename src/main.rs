mod point;
mod point_manager;

use point_manager::PointManager;

fn main() {

    let pm = PointManager::new(10);

    pm.print_points();

}
