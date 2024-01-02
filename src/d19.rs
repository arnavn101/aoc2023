use std::{
    cmp,
    collections::{HashMap, HashSet},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Condition {
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Always,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Rule {
    variable: String,
    value: usize,
    condition: Condition,
    next_workflow: NextWorkflow,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum NextWorkflow {
    Workflow(String),
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Clone, Debug)]
struct Rating {
    parts: HashMap<String, usize>,
}

fn parse_lines(lines: &Vec<String>) -> (HashMap<String, Workflow>, Vec<Rating>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut ratings: Vec<Rating> = Vec::new();

    let mut parse_ratings = false;

    for line in lines.iter() {
        if line.is_empty() {
            parse_ratings = true;
            continue;
        }

        if !parse_ratings {
            let mut parts = line.split('{');
            let wflow_name = parts.next().unwrap().trim().to_string();
            let wflow_contents = parts
                .next()
                .unwrap()
                .trim()
                .strip_suffix('}')
                .unwrap()
                .to_string();
            let content_parts = wflow_contents.split(',').collect::<Vec<&str>>();
            let wflow = Workflow {
                rules: content_parts
                    .iter()
                    .map(|x| {
                        fn parse_workflow(x: &str) -> NextWorkflow {
                            if x == "A" {
                                return NextWorkflow::Accepted;
                            } else if x == "R" {
                                return NextWorkflow::Rejected;
                            } else {
                                return NextWorkflow::Workflow(x.to_string());
                            }
                        }

                        if !x.contains(':') {
                            return Rule {
                                variable: "".to_string(),
                                value: 0,
                                condition: Condition::Always,
                                next_workflow: parse_workflow(x),
                            };
                        }

                        let mut parts = x.split(':');
                        let variable_and_condition = parts.next().unwrap().trim();
                        let next_workflow = parse_workflow(parts.next().unwrap().trim());

                        if variable_and_condition.contains('>') {
                            let mut parts = variable_and_condition.split('>');
                            let variable = parts.next().unwrap().trim().to_string();
                            let value = parts.next().unwrap().trim().parse::<usize>().unwrap();
                            Rule {
                                variable,
                                value,
                                condition: Condition::GreaterThan,
                                next_workflow,
                            }
                        } else if variable_and_condition.contains('<') {
                            let mut parts = variable_and_condition.split('<');
                            let variable = parts.next().unwrap().trim().to_string();
                            let value = parts.next().unwrap().trim().parse::<usize>().unwrap();
                            Rule {
                                variable,
                                value,
                                condition: Condition::LessThan,
                                next_workflow,
                            }
                        } else {
                            panic!("Invalid rule: {}", x);
                        }
                    })
                    .collect::<Vec<Rule>>(),
            };
            workflows.insert(wflow_name, wflow);
        } else {
            let clean_rating = line
                .trim()
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .to_string();
            let rating_parts = clean_rating.split(',');
            let mut rating = Rating {
                parts: HashMap::new(),
            };

            for part in rating_parts {
                let mut parts = part.split('=');
                let variable = parts.next().unwrap().trim().to_string();
                let value = parts.next().unwrap().trim().parse::<usize>().unwrap();
                rating.parts.insert(variable, value);
            }

            ratings.push(rating);
        }
    }

    (workflows, ratings)
}

fn find_accepted_wflows(
    workflows: &HashMap<String, Workflow>,
    ratings: &Vec<Rating>,
) -> HashSet<usize> {
    let mut accepted_ratings: HashSet<usize> = HashSet::new();

    for (rating_idx, cur_rating) in ratings.iter().enumerate() {
        let mut cur_wflow = NextWorkflow::Workflow(String::from("in"));
        let map_variables = cur_rating
            .parts
            .keys()
            .map(|s| s.clone())
            .collect::<Vec<String>>();

        loop {
            let cur_wflow_name = match cur_wflow {
                NextWorkflow::Workflow(ref name) => name,
                NextWorkflow::Accepted => {
                    accepted_ratings.insert(rating_idx);
                    break;
                }
                NextWorkflow::Rejected => {
                    break;
                }
            };

            let cur_wflow_rules = workflows.get(cur_wflow_name).unwrap();

            for rule in cur_wflow_rules.rules.iter() {
                match rule.condition {
                    Condition::Always => {
                        cur_wflow = rule.next_workflow.clone();
                        break;
                    }
                    _ => {
                        if map_variables.contains(&rule.variable) {
                            let cur_rating_value = cur_rating.parts.get(&rule.variable).unwrap();
                            let rule_value = rule.value;

                            match rule.condition {
                                Condition::LessThan => {
                                    if cur_rating_value < &rule_value {
                                        cur_wflow = rule.next_workflow.clone();
                                        break;
                                    }
                                }
                                Condition::GreaterThan => {
                                    if cur_rating_value > &rule_value {
                                        cur_wflow = rule.next_workflow.clone();
                                        break;
                                    }
                                }
                                _ => panic!("Invalid condition"),
                            }
                        }
                    }
                }
            }
        }
    }

    accepted_ratings
}

fn aggregate_accepted_ratings(ratings: &Vec<Rating>, accepted_ratings: &HashSet<usize>) -> usize {
    ratings.iter().enumerate().fold(0, |acc, (idx, rating)| {
        if accepted_ratings.contains(&idx) {
            acc + rating.parts.values().sum::<usize>()
        } else {
            acc
        }
    })
}

fn find_pos_combs(workflows: &HashMap<String, Workflow>) -> usize {
    let mut all_combs = 0;

    // x, m, a ,s
    let mut stack = vec![(
        NextWorkflow::Workflow(String::from("in")),
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
    )];

    while !stack.is_empty() {
        let (
            wflow,
            (mut x_low, mut x_high),
            (mut m_low, mut m_high),
            (mut a_low, mut a_high),
            (mut s_low, mut s_high),
        ) = stack.pop().unwrap();

        if x_low > x_high || m_low > m_high || a_low > a_high || s_low > s_high {
            continue;
        }

        match wflow {
            NextWorkflow::Accepted => {
                all_combs += (x_high - x_low + 1)
                    * (m_high - m_low + 1)
                    * (a_high - a_low + 1)
                    * (s_high - s_low + 1);
            }
            NextWorkflow::Rejected => (),
            NextWorkflow::Workflow(wflow_name) => {
                let wflow = workflows.get(&wflow_name).unwrap();

                for rule in wflow.rules.iter() {
                    match rule.condition {
                        Condition::Always => {
                            stack.push((
                                rule.next_workflow.clone(),
                                (x_low, x_high),
                                (m_low, m_high),
                                (a_low, a_high),
                                (s_low, s_high),
                            ));
                        }
                        _ => {
                            fn update_range(
                                value: usize,
                                condition: Condition,
                                mut low: usize,
                                mut high: usize,
                            ) -> (usize, usize) {
                                match condition {
                                    Condition::LessThan => {
                                        high = cmp::min(value - 1, high);
                                    }
                                    Condition::GreaterThan => {
                                        low = cmp::max(value + 1, low);
                                    }
                                    Condition::LessThanEqual => {
                                        high = cmp::min(value, high);
                                    }
                                    Condition::GreaterThanEqual => {
                                        low = cmp::max(value, low);
                                    }
                                    _ => panic!("Invalid condition"),
                                }

                                (low, high)
                            }

                            fn update_ranges(
                                variable: String,
                                value: usize,
                                condition: Condition,
                                mut x_low: usize,
                                mut x_high: usize,
                                mut m_low: usize,
                                mut m_high: usize,
                                mut a_low: usize,
                                mut a_high: usize,
                                mut s_low: usize,
                                mut s_high: usize,
                            ) -> (usize, usize, usize, usize, usize, usize, usize, usize)
                            {
                                match variable.as_str() {
                                    "x" => {
                                        (x_low, x_high) =
                                            update_range(value, condition, x_low, x_high);
                                    }
                                    "m" => {
                                        (m_low, m_high) =
                                            update_range(value, condition, m_low, m_high);
                                    }
                                    "a" => {
                                        (a_low, a_high) =
                                            update_range(value, condition, a_low, a_high);
                                    }
                                    "s" => {
                                        (s_low, s_high) =
                                            update_range(value, condition, s_low, s_high);
                                    }
                                    _ => panic!("Invalid variable"),
                                }

                                (x_low, x_high, m_low, m_high, a_low, a_high, s_low, s_high)
                            }

                            let (
                                x_low_2,
                                x_high_2,
                                m_low_2,
                                m_high_2,
                                a_low_2,
                                a_high_2,
                                s_low_2,
                                s_high_2,
                            ) = update_ranges(
                                rule.variable.clone(),
                                rule.value,
                                rule.condition.clone(),
                                x_low,
                                x_high,
                                m_low,
                                m_high,
                                a_low,
                                a_high,
                                s_low,
                                s_high,
                            );
                            stack.push((
                                rule.next_workflow.clone(),
                                (x_low_2, x_high_2),
                                (m_low_2, m_high_2),
                                (a_low_2, a_high_2),
                                (s_low_2, s_high_2),
                            ));

                            (x_low, x_high, m_low, m_high, a_low, a_high, s_low, s_high) =
                                update_ranges(
                                    rule.variable.clone(),
                                    rule.value,
                                    if rule.condition == Condition::GreaterThan {
                                        Condition::LessThanEqual
                                    } else {
                                        Condition::GreaterThanEqual
                                    },
                                    x_low,
                                    x_high,
                                    m_low,
                                    m_high,
                                    a_low,
                                    a_high,
                                    s_low,
                                    s_high,
                                );
                        }
                    }
                }
            }
        }
    }

    all_combs
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (workflows, ratings) = parse_lines(lines);
    let accepted_ratings = find_accepted_wflows(&workflows, &ratings);
    aggregate_accepted_ratings(&ratings, &accepted_ratings)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (workflows, _) = parse_lines(lines);
    find_pos_combs(&workflows)
}
