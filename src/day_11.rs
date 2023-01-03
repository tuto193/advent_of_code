use std::{cell::RefCell, rc::Rc};

use crate::get_file_contents;

mod monkey;

use monkey::Monkey;

pub fn day_11() {
    let input = get_file_contents("11".into());
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut actual_monkeys: Vec<Monkey> = input.into_iter().map(|monkey_lines|
        Monkey::from_input(monkey_lines.into())
    ).collect();
    let mut monkeys: Rc<RefCell<&mut Vec<Monkey>>> = Rc::new(RefCell::new(&mut actual_monkeys));
    let expected_rounds: usize = 20;
    let mut monkey_reference_list = Rc::clone(&monkeys);
    for _round in 0..expected_rounds {
        // Play a round
        for monkey in monkeys.borrow_mut().into_iter() {
            monkey.operation(&mut monkey_reference_list);
        }
    }
    // println!("Base monkeys:     \t{:?}", actual_monkeys);
    // println!("Reference monkeys:\t{:?}", monkeys.borrow_mut());
    let mut total_passes: Vec<usize> = actual_monkeys.into_iter().map(|m| m.get_times_inspecting_items()).collect();
    total_passes.sort();
    let most_pases = total_passes.pop().unwrap();
    let second_most_pases = total_passes.pop().unwrap();
    let money_business = most_pases * second_most_pases;
    println!("Monkey business is: {}", money_business);
}
