use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug)]
enum Module {
    FlipFlop {
        state: bool,
        destination_modules: Vec<String>,
    },
    Conjunction {
        input_pulses: HashMap<String, Pulse>,
        destination_modules: Vec<String>,
    },
    Broadcast {
        destination_modules: Vec<String>,
    },
    NOOP,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Pulse {
    High,
    Low,
}

fn parse_lines(lines: &Vec<String>) -> (HashMap<String, Module>, HashMap<String, HashSet<String>>) {
    let mut modules = HashMap::new();
    let mut to_from_modules: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        let mut split = line.split("->");
        let mut module_name = split.next().unwrap().trim().to_string();
        let module_outputs: Vec<String> = split
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();

        let mut module = Module::NOOP;

        if module_name == "broadcaster" {
            module = Module::Broadcast {
                destination_modules: module_outputs.clone(),
            };
        } else if module_name.starts_with('%') {
            module_name = module_name[1..].to_string();
            module = Module::FlipFlop {
                state: false,
                destination_modules: module_outputs.clone(),
            };
        } else if module_name.starts_with('&') {
            module_name = module_name[1..].to_string();
            module = Module::Conjunction {
                input_pulses: HashMap::new(),
                destination_modules: module_outputs.clone(),
            };
        }

        modules.insert(module_name.clone(), module);

        for output in module_outputs.iter() {
            to_from_modules
                .entry(output.clone())
                .or_insert_with(HashSet::new)
                .insert(module_name.clone());
        }
    }

    for (to_module_name, from_module_names) in to_from_modules.iter() {
        for from_module_name in from_module_names.iter() {
            if !modules.contains_key(to_module_name) {
                let module = Module::NOOP;
                modules.insert(to_module_name.clone(), module);
            }

            let to_module = modules.get_mut(to_module_name).unwrap();

            match to_module {
                Module::Conjunction {
                    input_pulses,
                    destination_modules: _,
                } => {
                    input_pulses.insert(from_module_name.clone(), Pulse::Low);
                }
                _ => {}
            }
        }
    }

    (modules, to_from_modules)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(nums: Vec<usize>) -> usize {
    let mut lcm = nums[0];
    for i in 1..nums.len() {
        lcm = lcm * nums[i] / gcd(lcm, nums[i]);
    }
    lcm
}

fn simulate_pulses(
    mut modules: HashMap<String, Module>,
    to_from_modules: HashMap<String, HashSet<String>>,
    num_times: usize,
    is_p2: bool,
) -> usize {
    let (mut num_low_pulses, mut num_high_pulses) = (num_times, 0);

    // assume final_module is conjunction module
    let final_module = to_from_modules.get("rx").unwrap().iter().next().unwrap();
    let mut before_final_module = to_from_modules.get(final_module).unwrap().clone();
    let mut length_cycles: Vec<usize> = Vec::new();

    for button_press_num in 1..num_times + 1 {
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        queue.push_back((
            String::from("button"),
            String::from("broadcaster"),
            Pulse::Low,
        ));

        while !queue.is_empty() {
            let (from_module_name, module_name, pulse) = queue.pop_front().unwrap();
            let module: &Module = modules.get(&module_name).unwrap_or_else(|| {
                panic!(
                    "Module {} does not exist. From module: {}",
                    module_name, from_module_name
                )
            });

            match module.clone() {
                Module::Broadcast {
                    destination_modules,
                } => {
                    assert!(pulse == Pulse::Low);
                    num_low_pulses += destination_modules.len() * !is_p2 as usize;

                    for destination in destination_modules.iter() {
                        queue.push_back((module_name.clone(), destination.clone(), Pulse::Low));
                    }
                }
                Module::FlipFlop {
                    state,
                    destination_modules,
                } => {
                    if pulse == Pulse::Low {
                        let to_send_pulse = if state {
                            num_low_pulses += destination_modules.len() * !is_p2 as usize;
                            Pulse::Low
                        } else {
                            num_high_pulses += destination_modules.len() * !is_p2 as usize;
                            Pulse::High
                        };

                        modules
                            .get_mut(&module_name)
                            .unwrap()
                            .clone_from(&Module::FlipFlop {
                                state: !state,
                                destination_modules: destination_modules.clone(),
                            });

                        for destination in destination_modules.iter() {
                            queue.push_back((
                                module_name.clone(),
                                destination.clone(),
                                to_send_pulse,
                            ));
                        }
                    }
                }
                Module::Conjunction {
                    mut input_pulses,
                    destination_modules,
                } => {
                    input_pulses
                        .get_mut(&from_module_name)
                        .unwrap_or_else(|| {
                            panic!(
                                "Module {} does not exist. From conjunction module: {}",
                                from_module_name, module_name
                            )
                        })
                        .clone_from(&pulse);

                    let to_send_pulse =
                        if input_pulses.iter().all(|(_, pulse)| *pulse == Pulse::High) {
                            num_low_pulses += destination_modules.len() * !is_p2 as usize;
                            Pulse::Low
                        } else {
                            num_high_pulses += destination_modules.len() * !is_p2 as usize;
                            Pulse::High
                        };

                    modules
                        .get_mut(&module_name)
                        .unwrap()
                        .clone_from(&Module::Conjunction {
                            input_pulses: input_pulses.clone(),
                            destination_modules: destination_modules.clone(),
                        });

                    for destination in destination_modules.iter() {
                        queue.push_back((module_name.clone(), destination.clone(), to_send_pulse));
                    }

                    if is_p2
                        && before_final_module.contains(&module_name)
                        && to_send_pulse == Pulse::High
                    {
                        length_cycles.push(button_press_num);
                        before_final_module.remove(&module_name);
                    }
                }
                Module::NOOP => {}
            }

            if is_p2 && before_final_module.is_empty() {
                return lcm(length_cycles);
            }
        }
    }

    num_low_pulses * num_high_pulses
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (modules, to_from_modules) = parse_lines(lines);
    simulate_pulses(modules, to_from_modules, 1000, false)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (modules, to_from_modules) = parse_lines(lines);
    simulate_pulses(modules, to_from_modules, usize::MAX - 1, true)
}
