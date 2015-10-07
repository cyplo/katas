use std::collections::HashSet;

struct WorldMap{
    map: HashSet<(i8, i8)>
}

impl WorldMap{
    
    fn new() -> WorldMap {
        WorldMap { map: HashSet::new() }
    }
   
    // make self immutable here, return a new copy of World
    fn mark_cell_as_alive(& mut self, cell_coordinates: (i8, i8)) {
        self.map.insert(cell_coordinates);
    }

    fn is_cell_alive(&self, cell_coordinates: (i8,i8)) -> bool {
        return self.map.contains(&cell_coordinates);
    }

}

#[test]
fn is_cell_alive_should_return_true_for_alive_cell(){
    let coords = (0,0);
    let mut world = WorldMap::new();
    world.mark_cell_as_alive(coords);
    let is_alive = world.is_cell_alive(coords);
    assert_eq!(true, is_alive);
}

#[test]
fn is_cell_alive_should_return_false_for_dead_cell(){
    let coords = (0,0);
    let world = WorldMap::new();
    let is_alive = world.is_cell_alive(coords);
    assert_eq!(false, is_alive);
}
