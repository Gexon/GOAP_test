extern crate rgoap;

use rgoap::{State, Action, plan};


fn main() {
    println!("Hello, GOAP!");

    // Действия использующиеся в планировщике.
    let mut walk_to_dog = Action::new("Идем к собакену".to_string(), 1);
    walk_to_dog.pre_conditions.insert("человек с собакой".to_string(), true);
    walk_to_dog.post_conditions.insert("рядом с собакой".to_string(), true);

    let mut dog_wiggles_tail = Action::new("Псина виляет хвостом".to_string(), 1);
    dog_wiggles_tail.pre_conditions.insert("пес счастлив".to_string(), true);
    dog_wiggles_tail.post_conditions.insert("Хвост колышется".to_string(), true);

    let mut pet_dog = Action::new("Погладить псину".to_string(), 1);
    pet_dog.pre_conditions.insert("рядом с собакой".to_string(), true);
    pet_dog.post_conditions.insert("пес счастлив".to_string(), true);

    let mut kill_dog = Action::new("Выстрелить в псину".to_string(), 1);
    kill_dog.pre_conditions.insert("ствол есть".to_string(), true);
    kill_dog.post_conditions.insert("пес мертв".to_string(), true);

    let mut find_weapon = Action::new("Найти ствол".to_string(), 1);
    find_weapon.pre_conditions.insert("ствол есть".to_string(), false);
    find_weapon.post_conditions.insert("ствол есть".to_string(), true);

    let possible_actions = [walk_to_dog, pet_dog, dog_wiggles_tail, kill_dog, find_weapon];

    // Это начальное состояние мира.
    let mut initial_state = State::new();
    initial_state.insert("рядом с собакой".to_string(), false);
    initial_state.insert("человек с собакой".to_string(), true);
    initial_state.insert("пес счастлив".to_string(), false);
    initial_state.insert("Хвост колышется".to_string(), false);
    initial_state.insert("пес мертв".to_string(), false);
    initial_state.insert("ствол есть".to_string(), false);

    // А это, целевое состояние. Не должен включать все состояния.
    let mut goal_state = State::new();
    goal_state.insert("пес мертв".to_string(), true);

    // Давайте найдем, какие действия нужно сделать, чтобы попасть туда.
    let planned_actions = plan(&initial_state, &goal_state, &possible_actions).unwrap();

    // Действия то, что мы ожидали?
    let planned_actions_names: Vec<String> =
        planned_actions.iter().map(|&action| action.name.clone()).collect();
    //let expected_actions_names =
    //    vec!["walk_to_dog".to_string(), "pet_dog".to_string(), "dog_wiggles_tail".to_string()];
    //assert_eq!(planned_actions_names, expected_actions_names);
    println!("Цель планировщика: {:?}", goal_state);
    println!("Цепь действий: {:?}", planned_actions_names);
}



