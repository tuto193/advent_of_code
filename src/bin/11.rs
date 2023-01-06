use std::{borrow::Borrow, cell::RefCell, rc::Rc};
use advent_of_code::helpers::monkey::Monkey;

pub fn part_one(input: &str) -> Option<u128> {
    let input: Vec<&str> = input.split("\n\n").collect();
    let monkeys: Rc<Vec<Rc<RefCell<Monkey>>>> = Rc::new(
        input
            .into_iter()
            .map(|monkey_lines| Rc::new(RefCell::new(Monkey::from_input(monkey_lines.into(), 3))))
            .collect(),
    );
    let expected_rounds: usize = 20;
    let monkey_reference_list = Rc::clone(&monkeys);
    for _round in 0..expected_rounds {
        // Play a round
        let monkeys = Rc::clone(&monkeys);
        for monkey in monkeys.iter() {
            Rc::clone(monkey)
                .borrow_mut()
                .operation(monkey_reference_list.borrow());
        }
    }
    // println!("Base monkeys:     \t{:?}", actual_monkeys);
    // println!("Reference monkeys:\t{:?}", monkeys.borrow_mut());
    let mut total_passes: Vec<u128> = monkeys
        .iter()
        .map(|m| Rc::clone(&m).borrow_mut().get_times_inspecting_items())
        .collect();
    println!("Total passes: {:?}", total_passes);
    total_passes.sort();
    println!("Sorted passes: {:?}", total_passes);
    let most_pases = total_passes.pop().unwrap();
    let second_most_pases = total_passes.pop().unwrap();
    let money_business = most_pases * second_most_pases;
    // println!("Monkey business is: {}", money_business);
    // println!("Remaining passes: {:?}", total_passes);

    Some(money_business)
}

pub fn part_two(input: &str) -> Option<u128> {
    let input: Vec<&str> = input.split("\n\n").collect();
    let monkeys: Rc<Vec<Rc<RefCell<Monkey>>>> = Rc::new(
        input
            .into_iter()
            .map(|monkey_lines| Rc::new(RefCell::new(Monkey::from_input(monkey_lines.into(), 1))))
            .collect(),
    );
    let new_fear: u128 = monkeys
        .clone()
        .iter()
        .map(|m| {
            let m_fear = m.try_borrow().unwrap().get_minus_fear();
            println!("m_fear is {}", m_fear);
            m_fear
        })
        .product();
    let expected_rounds: usize = 10000;
    let monkey_reference_list = Rc::clone(&monkeys);
    // make sure they all have a common fear
    for monkey in Rc::clone(&monkeys).iter() {
        Rc::clone(monkey).borrow_mut().change_minus_fear(new_fear);
    }

    // Play rounds normally
    for _round in 0..expected_rounds {
        // Play a round
        let monkeys = Rc::clone(&monkeys);
        for monkey in monkeys.iter() {
            Rc::clone(monkey)
                .borrow_mut()
                .operation(monkey_reference_list.borrow());
        }
    }
    // println!("Base monkeys:     \t{:?}", actual_monkeys);
    // println!("Reference monkeys:\t{:?}", monkeys.borrow_mut());
    let mut total_passes: Vec<u128> = monkeys
        .iter()
        .map(|m| Rc::clone(&m).borrow_mut().get_times_inspecting_items())
        .collect();
    // println!("Total passes: {:?}", total_passes);
    total_passes.sort();
    // println!("Sorted passes: {:?}", total_passes);
    let most_pases = total_passes.pop().unwrap();
    let second_most_pases = total_passes.pop().unwrap();
    let money_business = most_pases * second_most_pases;
    // println!("Monkey business with lots of fear is: {}", money_business);
    Some(money_business)

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
