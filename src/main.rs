extern crate flat_map;

mod cell;

use cell::{Cell, CellState};
use flat_map::FlatMap;





fn main() {
	 let rows = 10; let cols = 10;
	 let mut m: FlatMap<(i32, i32), Cell> = FlatMap::new();
	 
	 
	 for y in 0..rows {
	    for x in 0..cols {
	 		m.insert((y, x), Cell::new(&CellState::Dead));
	 	}	 	
	 }
    
    assert_eq!(m.iter().count(), 100);
    
    let mut current_row = (m.iter().next().unwrap().0).0;
    for cell in m.iter() {
    	
    	let y = &(cell.0).0;
    	if y != &current_row {
    		print!("\n");
    		current_row = *y;
    	}
    								
    	print!("  ({x:} , {y:})  ", 
    		x=(cell.0).1,
    		y=(cell.0).0
    	); 
    }
    
}

fn apply_generation(map: FlatMap<(i32, i32), Cell>, bounds: (&i32, &i32)){
	
	for cell in map.iter() {
		let (x, y): (&i32, &i32) = (&(cell.0).1, &(cell.0).0);
		let c: &Cell	= &cell.1;
		
		if x == bounds.0 || y == bounds.1 { continue; }
		
		
		// TODO: Find neighbor cells
		//		 Enqueue them.
		//		 Apply Generation to the cell. 
		
		
		
		
	}
	
}








