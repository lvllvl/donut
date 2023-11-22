pub const GRID_WIDTH: i32 = 21;
pub const GRID_HEIGHT: i32 = GRID_WIDTH;
pub const RADIUS: i32 = calculate_radius(GRID_WIDTH, GRID_HEIGHT);
pub const WIDTH: i32 = RADIUS * 2;
pub const HEIGHT: i32 = RADIUS; 

const fn calculate_radius( width: i32, height: i32 ) -> i32 {

    if width < 0 || height < 0 {
        panic!("Width and height must be positive integers");
    } else if width < height {
        width / 2
    } else {
        height / 2
    }

}