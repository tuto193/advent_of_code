use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum OperationType {
    Sum,
    Product,
}

#[derive(Debug)]
pub struct Monkey {
    monkey_index: usize,
    items: Vec<u128>,
    operation: (OperationType, Option<u128>),
    test_modulator: u128,
    throw_to_monkeys: Vec<usize>,
    times_inspected_items: u128,
    minus_fear: u128,
}

impl Monkey {
    pub fn from_input(input: String, m_fear: u128) -> Self {
        let input: Vec<&str> = input.split("\n").collect();
        // Read the index
        let monkey_line: Vec<&str> = input[0].split(" ").collect();
        let monkey_line: Vec<&str> = monkey_line[1].split(":").collect();
        let read_index: usize = monkey_line[0].parse().unwrap();

        // Get the starting items
        let items_line: Vec<&str> = input[1].split(": ").collect();
        let items_line: Vec<&str> = items_line[1].split(", ").collect();
        let items: Vec<u128> = items_line.into_iter().map(|n| n.parse().unwrap()).collect();

        // Get the operation stuff
        let operation_line: Vec<&str> = input[2].split("= ").collect();
        let operation_line: Vec<&str> = operation_line[1].split(" ").collect();
        let opt_type_l = match operation_line[1] {
            "+" => OperationType::Sum,
            _ => OperationType::Product,
        };
        let opt_type_r: Option<u128> = match operation_line[2] {
            "old" => None,
            x => Some(x.parse().unwrap()),
        };

        // Get Test stuff
        let test_line: Vec<&str> = input[3].split("by ").collect();
        let test_mod: u128 = test_line[1].parse().unwrap();

        // Get Other monkeys' line
        let monkey_1: Vec<&str> = input[4].split("monkey ").collect();
        let monkey_1: usize = monkey_1[1].parse().unwrap();
        let monkey_2: Vec<&str> = input[5].split("monkey ").collect();
        let monkey_2: usize = monkey_2[1].parse().unwrap();
        let throw_to_monk: Vec<usize> = vec![monkey_1, monkey_2];

        // Make the actual monkey
        Self {
            monkey_index: read_index,
            items: items,
            operation: (opt_type_l, opt_type_r),
            test_modulator: test_mod,
            throw_to_monkeys: throw_to_monk,
            times_inspected_items: 0,
            minus_fear: m_fear,
        }
    }

    pub fn operation(&mut self, monkey_list: &Vec<Rc<RefCell<Monkey>>>) {
        while let Some(old) = self.items.pop() {
            // Inspect items
            let do_self = self.operation.1 == None;
            let new = match self.operation.0 {
                OperationType::Sum => {
                    if do_self {
                        old * old
                    } else {
                        old + self.operation.1.unwrap()
                    }
                }
                OperationType::Product => {
                    if do_self {
                        old * old
                    } else {
                        old * self.operation.1.unwrap()
                    }
                }
            };
            // Done inspecting
            self.times_inspected_items += 1;
            let minus_worry = if self.minus_fear == 3 {
                new / self.minus_fear
            } else {
                new % self.minus_fear
            };
            let throw_to: usize = if self.test(minus_worry.clone()) { 0 } else { 1 };
            monkey_list[self.throw_to_monkeys[throw_to]]
                .borrow_mut()
                .items
                .push(minus_worry);
        }
    }

    pub fn test(&mut self, item: u128) -> bool {
        item % self.test_modulator == 0
    }
    pub fn get_times_inspecting_items(&self) -> u128 {
        self.times_inspected_items
    }

    pub fn change_minus_fear(&mut self, new: u128) {
        self.minus_fear = new;
    }

    pub fn get_minus_fear(&self) -> u128 {
        self.test_modulator
    }
}
