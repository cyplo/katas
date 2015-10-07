use std::collections::HashSet;

struct WorldMap{
    map: HashSet<(i8, i8)>
}

impl WorldMap{
    
    fn empty() -> WorldMap {
        return WorldMap::new( HashSet::new() );
    }
    
    fn new(new_map: HashSet<(i8,i8)>) -> WorldMap {
        return WorldMap { map: new_map };
    }

    fn mark_cell_as_alive(&self, cell_coordinates: (i8, i8)) -> WorldMap {
        let mut new_map = self.map.clone();
        new_map.insert(cell_coordinates);
        let new_world = WorldMap::new(new_map); 
        return new_world;
    }

    fn is_cell_alive(&self, cell_coordinates: (i8,i8)) -> bool {
        return self.map.contains(&cell_coordinates);
    }

}

#[test]
fn is_cell_alive_should_return_true_for_alive_cell(){
    let coords = (0,0);
    let world = WorldMap::empty();
    let new_world = world.mark_cell_as_alive(coords);
    let is_alive = new_world.is_cell_alive(coords);
    assert_eq!(true, is_alive);
}

#[test]
fn is_cell_alive_should_return_false_for_dead_cell(){
    let coords = (0,0);
    let world = WorldMap::empty();
    let is_alive = world.is_cell_alive(coords);
    assert_eq!(false, is_alive);
}
