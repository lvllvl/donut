use std::io::Write;

use crossterm::{
    cursor::{self, MoveTo},
    execute,
    terminal::{self, ClearType, size},
    ExecutableCommand,
};

mod config; 

fn main() {
    // Crossterm setup
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    stdout.execute(terminal::Clear(ClearType::All)).expect("Failed to clear screen");
    stdout.execute(cursor::Hide).expect("Failed to hide cursor");
    
    let ( circle_width, circle_height ) = ( config::RADIUS * 2, config::RADIUS ); // Circle dimensions
    let ( term_width, term_height ) = match terminal::size() {
        Ok( size ) => (size.0, size.1 ), 
        Err( _ ) => ( 0, 0 ), // Handle the Error case
    };

    // Calculate the starting positions
    let start_x = ( term_width - circle_width as u16 ) / 2;  
    let start_y = ( term_height - circle_height as u16 ) / 2;  

    // Create and initialize the 2D array
    let mut grid = vec![ vec![ '.'; config::WIDTH as usize ]; config::HEIGHT as usize ];

    let thickness = 1; // adjust thickness of the circle

    // Calculate and mark the circle points
    for x in -config::RADIUS..config::RADIUS {
        for y in -(config::RADIUS/2)..config::RADIUS/2 {

            let grid_x = (x + config::RADIUS) as usize;
            let grid_y = (y + config::RADIUS/2) as usize;

            let dist_from_center = x*x + y*y;
            let inner_radius = ( config::RADIUS- thickness ) * ( config::RADIUS- thickness ); 
            let outer_radius = ( config::RADIUS + thickness ) * ( config::RADIUS+ thickness );

            if inner_radius <= dist_from_center && dist_from_center <= outer_radius {
                grid[ grid_y ][ grid_x ] = '#'; 
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
