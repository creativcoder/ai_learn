// Represents each location as a node in the graph
struct Location {
	Name:&'static str,
	cost:u32,
}

enum ENVIRON_STATE {

}

type environment = [[u32;8];8];

// Action set to take 
enum ACTIONS {
	LEFT(i32),
	UP(i32),
	RIGHT(i32),
	DOWN(i32),
}

// hoisting inner types to use later
use ACTIONS::*;


// evaluates an action selected from an action vector
fn eval_action() {

}

// returns 
fn action(state:ENVIRON_STATE) -> Vec<ACTIONS>{
	vec![LEFT(3),RIGHT(7),DOWN(5),UP(4)]
}

fn result() -> bool {
	false
}

fn has_goal_state() -> bool {
	false
}

// places ai agent on the desired location
fn place_agent(envir:environment,loc:Location) {

}

fn main() {

	let mut grid:environment= [[0;8];8];
	for i in 0..grid.len() {
		for j in 0..grid[i].len() {
			grid[i][j] = 0;
		}
	}
	let route_graph:[Location;50];
	
}
