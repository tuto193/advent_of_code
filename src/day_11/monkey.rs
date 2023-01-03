use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
enum OperationType {
    Sum,
    Product,
}

#[derive(Debug)]
pub struct Monkey {
    monkey_index: usize,
    items: Vec<usize>,
    operation: (OperationType, Option<usize>),
    test_modulator: usize,
    throw_to_monkeys: Vec<usize>,
    times_inspected_items: usize,
}

impl Monkey {
    pub fn from_input(input: String) -> Self {
        let input: Vec<&str> = input.split("\n").collect();
        // Read the index
        let monkey_line: Vec<&str> = input[0].split(" ").collect();
        let monkey_line: Vec<&str> = monkey_line[1].split(":").collect();
        let read_index: usize = monkey_line[0].parse().unwrap();

        // Get the starting items
        let items_line: Vec<&str> = input[1].split(": ").collect();
        let items_line: Vec<&str> = items_line[1].split(", ").collect();
        let items: Vec<usize> = items_line.into_iter().map(|i| i.parse().unwrap()).collect();

        // Get the operation stuff
        let operation_line: Vec<&str> = input[2].split("= ").collect();
        let operation_line: Vec<&str> = operation_line[1].split(" ").collect();
        let opt_type_l = match operation_line[1] {
            "+" => OperationType::Sum,
            _ => OperationType::Product,
        };
        let opt_type_r: Option<usize> = match operation_line[2] {
            "old" => None,
            x => Some(x.parse().unwrap()),
        };

        // Get Test stuff
        let test_line: Vec<&str> = input[3].split("by ").collect();
        let test_mod: usize = test_line[1].parse().unwrap();

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
        }
    }

    pub fn operation(&mut self, monkey_list: &mut Rc<RefCell<&mut Vec<Monkey>>>) {
        while let Some(old) = self.items.pop() {
            // Inspect items
            let rhs = self.operation.1.unwrap_or(old);
            let new = match self.operation.0 {
                OperationType::Sum => old + rhs,
                OperationType::Product => old * rhs,
            };
            // Done inspecting
            self.times_inspected_items += 1;
            let minus_worry = new / 3;
            let throw_to: usize = if self.test(minus_worry) { 0 } else { 1 };
            monkey_list.borrow_mut()[self.throw_to_monkeys[throw_to]]
                .items
                .push(minus_worry);
        }
    }

    pub fn test(&mut self, item: usize) -> bool {
        item % self.test_modulator == 0
    }
    pub fn get_times_inspecting_items(&self) -> usize {
        self.times_inspected_items
    }
}
