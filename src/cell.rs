use std::fmt;

#[derive(Debug,Clone)]
pub enum CellState {
	Alive,
	Dead,
	BorderCell,
	Undefined
}


#[derive(Debug)]
pub struct Cell {
	state : CellState,
	next  : CellState
}


impl Cell {
	pub fn new(state : &CellState) -> Cell {
		Cell {
			 state : state.clone(),
			 next : CellState::Undefined
		}
	}
	
	fn set_state(&mut self, state : &CellState){
		self.state = state.clone();
	}
	
	fn set_next(&mut self, state : &CellState){
		self.next = state.clone();
	}
	pub fn set_border(&mut self){
		self.set_next(&CellState::BorderCell);
	}
	
	
	pub fn check(&mut self, others : &Vec<Cell>) {
		
		// guarantees fixed state of BorderCells
		match self.next {
			CellState::BorderCell => return,
			_					  => {}
		}
		
		
		let mut living = 0;
		
		for cell in others.iter() {
			match cell.state { 
				CellState::Alive => { living +=1 },
				_				 => {}
			}
		}
		
		match living {
			3 => self.set_next(&CellState::Alive),
			2 => match self.state {
				CellState::Alive => self.set_next(&CellState::Alive) ,
				_				=> {}
				
			},
			_ => self.set_next(&CellState::Dead)
		}
	}
	
	
	pub fn flush(&mut self) {
		assert!(match self.next {
				CellState::Undefined => false,
				_					 => true
				});
		match self.next {
			CellState::BorderCell => return,
			_					  => {}
		}
		
		let state = self.next.clone();
		self.set_state(&state);
		self.set_next(&CellState::Undefined);
	}
	
}


impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{}", 
        	match self.state {
    			CellState::Alive => "+",
    			CellState::Dead  => "-",
    			_				 => "?"		    		
	       	}
       	)
    }
	
}






