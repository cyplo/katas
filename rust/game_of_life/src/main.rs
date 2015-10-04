use std::collections::HashSet;

fn is_cell_alive(world: HashSet<(i8,i8)>, cell_coordinates: (i8,i8)) -> bool {
    return world.contains(&cell_coordinates);
}

#[test]
fn is_cell_alive_should_return_true_for_alive_cell(){
    let coords = (0,0);
    let mut world: HashSet<(i8,i8)> = HashSet::new();
    world.insert(coords);
    let is_alive = is_cell_alive(world,coords);
    assert_eq!(true, is_alive);
}

#[test]
fn is_cell_alive_should_return_false_for_dead_cell(){
    let coords = (0,0);
    let world: HashSet<(i8,i8)> = HashSet::new();
    let is_alive = is_cell_alive(world,coords);
    assert_eq!(false, is_alive);
}
