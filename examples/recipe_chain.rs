//! Look up production chains in the crafting graph.
//!
//! Run with:
//!     cargo run --example recipe_chain              # default: Hematite → PigIron
//!     cargo run --example recipe_chain StainlessSteel304

use crafting::{CraftingGraph, Substance};

fn main() {
    let target_name = std::env::args().nth(1).unwrap_or_else(|| "PigIron".into());

    let target = match parse_substance(&target_name) {
        Some(s) => s,
        None => {
            eprintln!("Unknown substance: {}", target_name);
            eprintln!("Try one of: PigIron, LowCarbonSteel, StainlessSteel304, Copper, Bronze");
            return;
        }
    };

    let graph = CraftingGraph::build_materials_only();
    println!(
        "Crafting graph: {} substances, {} recipe edges",
        graph.substance_count(),
        graph.edge_count()
    );
    println!();

    // What directly produces this substance?
    let inputs = graph.what_do_i_need(target);
    println!("Recipes producing {:?}:", target);
    if inputs.is_empty() {
        println!("  (this is a raw material — no inputs)");
    } else {
        for (from, recipe) in &inputs {
            println!("  {:?} → {:?} via {}", from, target, recipe);
        }
    }
    println!();

    // What can we make FROM this substance?
    let outputs = graph.what_can_i_make(target);
    if !outputs.is_empty() {
        println!("{:?} is an ingredient for:", target);
        for (to, recipe) in outputs.iter().take(10) {
            println!("  {:?} → {:?} via {}", target, to, recipe);
        }
        if outputs.len() > 10 {
            println!("  … and {} more", outputs.len() - 10);
        }
    }
}

fn parse_substance(name: &str) -> Option<Substance> {
    // Substance doesn't impl FromStr so we match by Debug string.
    for _s in [
        Substance::Hematite,
        Substance::Magnetite,
        Substance::PigIron,
        Substance::Iron,
        Substance::LowCarbonSteel,
        Substance::HighCarbonSteel,
        Substance::StainlessSteel304,
        Substance::Copper,
        Substance::Tin,
        Substance::TinBronze,
        Substance::Aluminum,
        Substance::Silicon,
        Substance::Gold,
        Substance::Silver,
        Substance::Water,
        Substance::Salt,
        Substance::Sulfur,
    ] {
        if format!("{:?}", _s) == name {
            return Some(_s);
        }
    }
    None
}
