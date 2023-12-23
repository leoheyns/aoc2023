use std::collections::{HashMap, VecDeque};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Pulse {
    HIGH,
    LOW,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum ModType {
    FLIPFLOP,
    CONJUNCTION,
    BROACASTER,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Module {
    name: &'static str,
    kind: ModType,
    targets: Vec<&'static str>,
}

pub fn run() {
    let input = include_str!("input")
        .lines()
        .map(|l| l.split(" -> "))
        .map(|mut splt| {
            (
                splt.next().unwrap(),
                splt.next().unwrap().split(", ").collect::<Vec<&str>>(),
            )
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    let mut modules: HashMap<&str, Module> = HashMap::new();

    let mut conjunction_states: HashMap<&str, HashMap<&str, Pulse>> = HashMap::new();

    let mut fliplop_states: HashMap<&str, bool> = HashMap::new();

    for (name, targets) in input {
        let mut final_name = name;
        let module = if name == "broadcaster" {
            Module {
                name: final_name,
                kind: ModType::BROACASTER,
                targets: targets,
            }
        } else if name.chars().next().unwrap() == '&' {
            final_name = &name[1..];
            conjunction_states.insert(final_name, HashMap::new());
            Module {
                name: final_name,
                kind: ModType::CONJUNCTION,
                targets: targets,
            }
        } else {
            final_name = &name[1..];
            Module {
                name: final_name,
                kind: ModType::FLIPFLOP,
                targets: targets,
            }
        };
        modules.insert(final_name, module);
    }

    for module in modules.values() {
        for target in &module.targets {
            match modules.get(target) {
                Some(t) => {
                    if t.kind == ModType::CONJUNCTION {
                        conjunction_states
                            .get_mut(target)
                            .unwrap()
                            .insert(module.name, Pulse::LOW);
                    }
                }
                None => {}
            }
        }
    }

    let mut pulse_queue: VecDeque<(Pulse, &str, &str)> = VecDeque::new(); //pulse, target, from

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for __ in 0..1000000 {
        // println!();
        pulse_queue.push_back((Pulse::LOW, "broadcaster", "button"));

        while pulse_queue.len() > 0 {
            let (pulse, target, from) = pulse_queue.pop_front().unwrap();
            // println!("{} -{:?}-> {}", from, pulse, target);
            match pulse {
                Pulse::HIGH => high_pulses += 1,
                Pulse::LOW => low_pulses += 1,
            }

            if target == "lx" && pulse == Pulse::HIGH {}

            if !modules.contains_key(target) {
                continue;
            }

            let target_module = modules.get(target).unwrap();

            // println!("{:?}", target_module);

            let out_pulse = match target_module.kind {
                ModType::BROACASTER => pulse,
                ModType::FLIPFLOP => {
                    if pulse == Pulse::HIGH {
                        continue;
                    }
                    let new_state = !fliplop_states.get(target).unwrap_or(&false);
                    fliplop_states.insert(target, new_state);
                    if new_state {
                        Pulse::HIGH
                    } else {
                        Pulse::LOW
                    }
                }
                ModType::CONJUNCTION => {
                    conjunction_states
                        .get_mut(target)
                        .unwrap()
                        .insert(from, pulse);
                    // println!("{:?}", target_module);
                    // println!("{:?}",  conjunction_states
                    // .get(target)
                    // .unwrap().iter().collect::<Vec<(&&str, &Pulse)>>());

                    if conjunction_states
                        .get(target)
                        .unwrap()
                        .values()
                        .all(|p| *p == Pulse::HIGH)
                    {
                        Pulse::LOW
                    } else {
                        Pulse::HIGH
                    }
                }
            };

            for new_target in &target_module.targets {
                pulse_queue.push_back((out_pulse.clone(), new_target, target))
            }
        }
    }

    println!("part1 {} {}", low_pulses, high_pulses);
    println!("{}", low_pulses * high_pulses);
}
