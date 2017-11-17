extern crate gpgoap;

use gpgoap::{WorldState, ActionPlanner, AStar};


fn main() {
    println!("Привет, GOAP!");

    // Действия использующиеся в планировщике.
    let mut ap = ActionPlanner::new();
    ap.set_pre("набить брюхо", "желудок урчит", true);
    ap.set_pre("набить брюхо", "еда перед аборигеном", true);
    ap.set_post("набить брюхо", "наелся от пуза", true);
    ap.set_post("набить брюхо", "еда перед аборигеном", false);

    ap.set_post("помолиться о еде", "еда перед аборигеном", true);

    ap.set_pre("убить краба", "рядом краб", true);
    ap.set_pre("убить краба", "камень зажат в руке", true);
    ap.set_post("убить краба", "еда перед аборигеном", true);

    ap.set_post("поиск крабов", "рядом краб", true);

    ap.set_post("поиск камня", "камень зажат в руке", true);

    ap.set_pre("выменять еды у товарища", "рядом другой абориген", true);
    ap.set_pre("выменять еды у товарища", "кость зажата в руке", true);
    ap.set_post("выменять еды у товарища", "еда перед аборигеном", true);
    ap.set_post("выменять еды у товарища", "кость зажата в руке", false);

    ap.set_post("поиск товарища", "рядом другой абориген", true);

    ap.set_post("поиск косточки", "кость зажата в руке", true);


    ap.set_cost("поиск крабов", 2);
    ap.set_cost("убить краба", 5);
    ap.set_cost("поиск камня", 3);
    ap.set_cost("поиск косточки", 8);
    ap.set_cost("помолиться о еде", 11);

    //print!("Planner:\n {:?}", &ap);

    let mut fr = WorldState::new();
    fr.set(&mut ap, "желудок урчит", true);
    fr.set(&mut ap, "alive", true);

    //print!("From: \n{:?}", fr.debug_fmt(&ap));

    let mut goal = WorldState::new();
    goal.set(&mut ap, "наелся от пуза", true);
    goal.set(&mut ap, "alive", true);

    //print!("Goal: \n{:?}", goal.debug_fmt(&ap));

    let mut astar = AStar::new();
    if let Some(plan) = astar.plan(&ap, &fr, &goal) {
        println!("Plan Cost: {}", plan.cost());
        for (i, &(plan, result_state)) in plan.iter().enumerate() {
            println!("{}: {}\n{:?}", i, plan, result_state.debug_fmt(&ap));
        }
    }
}



