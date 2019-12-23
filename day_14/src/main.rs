use std::collections::HashMap;

#[derive(Debug)]
struct Ingredient<'a> {
    pub name: &'a str,
    pub quantity: usize,
}

impl<'a> Ingredient<'a> {
    fn parse(spec: &'a str) -> Self {
        let parts: Vec<_> = spec.split_ascii_whitespace().collect();
        Ingredient {
            name: parts[1],
            quantity: parts[0].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Reaction<'a> {
    pub input: Vec<Ingredient<'a>>,
    pub output: Ingredient<'a>,
}

impl<'a> Reaction<'a> {
    fn parse(line: &'a str) -> Self {
        let parts: Vec<_> = line.split_terminator("=>").collect();
        if parts.len() != 2 {
            panic!("Unknown material: {}", line);
        }

        let input = parts[0].trim().split(',').map(Ingredient::parse).collect();
        let output = Ingredient::parse(parts[1].trim());
        Reaction {
            input: input,
            output: output,
        }
    }
}

fn ore_required<'a>(
    reactions: &Vec<Reaction<'a>>,
    want: &'a str,
    want_quantity: usize,
    leftover: &mut HashMap<&'a str, usize>,
) -> usize {
    if want == "ORE" {
        return want_quantity;
    }

    let r = reactions.iter().find(|r| r.output.name == want).unwrap();
    let mut ore_input = 0;
    let num_reactions = (want_quantity as f64 / r.output.quantity as f64).ceil() as usize;
    for ingredient in r.input.iter() {
        let mut quantity = ingredient.quantity * num_reactions;
        if let Some(&leftover_qty) = leftover.get(ingredient.name) {
            if leftover_qty > quantity {
                leftover.insert(ingredient.name, leftover_qty - quantity);
                continue;
            } else {
                quantity -= leftover_qty;
                leftover.remove(ingredient.name);
            }
        }
        ore_input += ore_required(reactions, ingredient.name, quantity, leftover);
    }
    let created = num_reactions * r.output.quantity;
    *leftover.entry(want).or_insert(0) += created - want_quantity;
    ore_input
}

fn main() {
    let input = include_str!("input").lines().map(Reaction::parse).collect();
    println!(
        "Required: {}",
        ore_required(&input, "FUEL", 1, &mut HashMap::new())
    );
}
