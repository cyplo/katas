use std::collections::HashSet;

struct WorldMap{
    map: HashSet<(i8, i8)>
}

fn get_neighbours_coordinates(cell_coordinates: (i8,i8)) -> HashSet<(i8,i8)> {
    let mut neighbour_coordinates = HashSet::new(); 
    let (x, y) = cell_coordinates;
    neighbour_coordinates.insert((x-1, y-1));
    neighbour_coordinates.insert((x-1, y));
    neighbour_coordinates.insert((x-1, y+1));
    neighbour_coordinates.insert((x, y-1));
    neighbour_coordinates.insert((x, y+1));
    neighbour_coordinates.insert((x+1, y-1));
    neighbour_coordinates.insert((x+1, y));
    neighbour_coordinates.insert((x+1, y+1));
    return neighbour_coordinates;
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


    fn count_alive_neighbours(&self, cell_coordinates: (i8, i8)) -> usize {
        let neighbour_coordinates = get_neighbours_coordinates(cell_coordinates);
        let neighbours_alive = self.map.intersection(&neighbour_coordinates).collect::<Vec<&(i8,i8)>>();
        return neighbours_alive.len();
    }
}

#[test]
fn the_only_alive_cell_should_have_no_alive_neighbours() {
    let coords = (0,0);
    let world = WorldMap::empty();
    let new_world = world.mark_cell_as_alive(coords);
    let alive_neighbours_count = new_world.count_alive_neighbours(coords);
    assert_eq!(0, alive_neighbours_count);
}

#[test]
fn alive_neighbour_on_the_diagonal_should_count() {
    let (x, y) = (0,0);
    let coords = (x,y);
    let world = WorldMap::empty();
    let new_world1 = world.mark_cell_as_alive(coords);
    let new_world2 = new_world1.mark_cell_as_alive((x, y+1));
    let alive_neighbours_count = new_world2.count_alive_neighbours(coords);
    assert_eq!(1, alive_neighbours_count);
}

#[test]
fn is_cell_alive_should_return_true_for_alive_cell() {
    let coords = (0,0);
    let world = WorldMap::empty();
    let new_world = world.mark_cell_as_alive(coords);
    let is_alive = new_world.is_cell_alive(coords);
    assert_eq!(true, is_alive);
}

#[test]
fn is_cell_alive_should_return_false_for_dead_cell() {
    let coords = (0,0);
    let world = WorldMap::empty();
    let is_alive = world.is_cell_alive(coords);
    assert_eq!(false, is_alive);
}
