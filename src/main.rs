use std::io::Write;

use crossterm::{
    cursor::{self, MoveTo},
    execute,
    terminal::{self, ClearType, size},
    ExecutableCommand,
};
mod config;
use config::{ GRID_HEIGHT, GRID_WIDTH, WIDTH, HEIGHT, RADIUS };

fn main() {
    // Crossterm setup
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    stdout.execute(terminal::Clear(ClearType::All)).expect("Failed to clear screen");
    stdout.execute(cursor::Hide).expect("Failed to hide cursor");
    
    let ( circle_width, circle_height ) = ( RADIUS * 2, RADIUS ); // Circle dimensions
    let ( term_width, term_height ) = match terminal::size() {
        Ok( size ) => (size.0, size.1 ), 
        Err( _ ) => ( 0, 0 ), // Handle the Error case
    };

    // Calculate the starting positions
    let start_x = ( term_width - circle_width as u16 ) / 2;  
    let start_y = ( term_height - circle_height as u16 ) / 2;  

    // Create and initialize the 2D array
    let mut grid = vec![ vec![ '.'; GRID_WIDTH as usize ]; GRID_HEIGHT as usize ];

    let thickness = 1; // adjust thickness of the circle

    // find the center of the grid (h,k)
    let h = GRID_WIDTH / 2;
    let k = GRID_HEIGHT / 2;

    // Adjust for aspect ration of chars in terminal 
    let aspect_ratio = 2.0;

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {

            // Adjust coordinate for aspect ratio
            let adjusted_x = (( x as f32 - h as f32 ) * aspect_ratio ).powi( 2 );
            let adjusted_y = ( y as f32 - k as f32 ).powi( 2 ); 

            let dist_from_center = adjusted_x + adjusted_y; 
            let radius_squared = ( RADIUS as f32 ).powi( 2 ); 
    
            let inner_radius = ( RADIUS as f32 - thickness as f32 ).powi(2); 
            let outer_radius = ( RADIUS as f32 + thickness as f32 ).powi(2);

            if inner_radius <= dist_from_center && dist_from_center <= outer_radius {
                grid[ y as usize ][ x as usize ] = '#'; 
            }

        }
    }

    render_circle( grid, start_x, start_y ); 
    terminal::disable_raw_mode().expect( "Failed to disable raw mode" );  
    println!("\nHello, DONUT!");

}

// Function to render the circle
fn render_circle( grid: Vec<Vec<char>>, start_x: u16, start_y: u16 ) {

    let mut stdout = std::io::stdout();

    for ( y, row ) in grid.iter().enumerate() {

        for ( x, col ) in row.iter().enumerate() {
            stdout.execute( cursor::MoveTo( start_x + x as u16 * 2, start_y + y as u16 ))
                .expect( "Failed to move cursor" );

            print!(" {}", col);
            // After print row, print a new line character
            println!();
        }
        // Flush stdout to ensure all chars are written to the terminal
        stdout.flush().expect( "Failed to flush stdout" ); 
    }
    stdout.execute( cursor::Show )
        .expect( "Failed to show cursor" );

}
