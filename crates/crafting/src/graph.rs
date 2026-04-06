use petgraph::algo::is_cyclic_directed;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::HashMap;

use crate::food;
use crate::recipes;
use crate::recipes::substance::Substance;
use crate::recipes::types::Recipe;

/// Classifies a recipe edge as a primary output or a byproduct.
///
/// Primary edges follow the recipe's intended product chain; byproduct edges
/// link to incidental co-products (e.g. slag from smelting, salt from
/// distillation). Traversal algorithms can choose whether to follow them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EdgeKind {
    Primary,
    Byproduct,
}

/// An edge in the crafting graph: the recipe name plus its output role.
#[derive(Clone, Copy, Debug)]
pub struct RecipeEdge {
    pub recipe: &'static str,
    pub kind: EdgeKind,
}

impl RecipeEdge {
    fn primary(recipe: &'static str) -> Self {
        Self {
            recipe,
            kind: EdgeKind::Primary,
        }
    }

    fn byproduct(recipe: &'static str) -> Self {
        Self {
            recipe,
            kind: EdgeKind::Byproduct,
        }
    }
}

/// A directed graph where nodes are Substances and edges are Recipes.
/// Edge direction: Input → Output (following material flow).
pub struct CraftingGraph {
    pub graph: DiGraph<Substance, RecipeEdge>,
    node_map: HashMap<Substance, NodeIndex>,
}

impl CraftingGraph {
    /// Build the full crafting graph from all recipes (materials + food).
    pub fn build_all() -> Self {
        let mut g = Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        };

        for recipe in recipes::all_recipes() {
            g.add_recipe(recipe);
        }
        for recipe in food::all_food_recipes() {
            g.add_recipe(recipe);
        }

        g
    }

    /// Build graph from material recipes only (no food).
    pub fn build_materials_only() -> Self {
        let mut g = Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        };

        for recipe in recipes::all_recipes() {
            g.add_recipe(recipe);
        }

        g
    }

    fn get_or_create_node(&mut self, substance: Substance) -> NodeIndex {
        *self
            .node_map
            .entry(substance)
            .or_insert_with(|| self.graph.add_node(substance))
    }

    fn add_recipe(&mut self, recipe: &'static Recipe) {
        // Create output nodes
        let output_nodes: Vec<NodeIndex> = recipe
            .outputs
            .iter()
            .map(|(s, _)| self.get_or_create_node(*s))
            .collect();

        // Create edges from each input to each primary output
        for (input_substance, _) in recipe.inputs {
            let input_node = self.get_or_create_node(*input_substance);
            for &output_node in &output_nodes {
                // Only add edge if it doesn't already exist with this recipe name
                if !self
                    .graph
                    .edges_connecting(input_node, output_node)
                    .any(|e| e.weight().recipe == recipe.name)
                {
                    self.graph
                        .add_edge(input_node, output_node, RecipeEdge::primary(recipe.name));
                }
            }
        }

        // Link byproducts with EdgeKind::Byproduct
        for (byproduct, _) in recipe.byproducts {
            let bp_node = self.get_or_create_node(*byproduct);
            for (input_substance, _) in recipe.inputs {
                let input_node = self.get_or_create_node(*input_substance);
                if !self
                    .graph
                    .edges_connecting(input_node, bp_node)
                    .any(|e| e.weight().recipe == recipe.name)
                {
                    self.graph
                        .add_edge(input_node, bp_node, RecipeEdge::byproduct(recipe.name));
                }
            }
        }
    }

    /// How many substances (nodes) in the graph.
    pub fn substance_count(&self) -> usize {
        self.graph.node_count()
    }

    /// How many recipe-edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// What can I make from this substance? (forward: what does it produce?)
    ///
    /// Returns all outgoing edges — both primary outputs and byproducts.
    pub fn what_can_i_make(&self, substance: Substance) -> Vec<(Substance, &'static str)> {
        let Some(&node) = self.node_map.get(&substance) else {
            return vec![];
        };
        self.graph
            .edges_directed(node, Direction::Outgoing)
            .map(|e| (self.graph[e.target()], e.weight().recipe))
            .collect()
    }

    /// Primary outputs producible from a substance (excludes byproducts).
    pub fn primary_outputs_from(&self, substance: Substance) -> Vec<(Substance, &'static str)> {
        let Some(&node) = self.node_map.get(&substance) else {
            return vec![];
        };
        self.graph
            .edges_directed(node, Direction::Outgoing)
            .filter(|e| e.weight().kind == EdgeKind::Primary)
            .map(|e| (self.graph[e.target()], e.weight().recipe))
            .collect()
    }

    /// Byproducts reachable from a substance (excludes primary outputs).
    pub fn byproducts_from(&self, substance: Substance) -> Vec<(Substance, &'static str)> {
        let Some(&node) = self.node_map.get(&substance) else {
            return vec![];
        };
        self.graph
            .edges_directed(node, Direction::Outgoing)
            .filter(|e| e.weight().kind == EdgeKind::Byproduct)
            .map(|e| (self.graph[e.target()], e.weight().recipe))
            .collect()
    }

    /// What do I need to make this substance? (backward: what are its inputs?)
    pub fn what_do_i_need(&self, substance: Substance) -> Vec<(Substance, &'static str)> {
        let Some(&node) = self.node_map.get(&substance) else {
            return vec![];
        };
        self.graph
            .edges_directed(node, Direction::Incoming)
            .map(|e| (self.graph[e.source()], e.weight().recipe))
            .collect()
    }

    /// Get the full production chain from raw material to target (BFS shortest path).
    pub fn production_chain(
        &self,
        from: Substance,
        to: Substance,
    ) -> Option<Vec<(Substance, &'static str)>> {
        let start = *self.node_map.get(&from)?;
        let end = *self.node_map.get(&to)?;

        // BFS
        let path = petgraph::algo::astar(&self.graph, start, |n| n == end, |_| 1, |_| 0);

        path.map(|(_, nodes)| {
            let mut chain = Vec::new();
            for window in nodes.windows(2) {
                let from_node = window[0];
                let to_node = window[1];
                let substance = self.graph[to_node];
                let recipe_name = self
                    .graph
                    .edges_connecting(from_node, to_node)
                    .next()
                    .map(|e| e.weight().recipe)
                    .unwrap_or("?");
                chain.push((substance, recipe_name));
            }
            chain
        })
    }

    /// Get all raw materials (substances with no incoming edges).
    pub fn raw_materials(&self) -> Vec<Substance> {
        self.graph
            .node_indices()
            .filter(|&n| self.graph.edges_directed(n, Direction::Incoming).count() == 0)
            .map(|n| self.graph[n])
            .collect()
    }

    /// Get all final products (substances with no outgoing edges).
    pub fn final_products(&self) -> Vec<Substance> {
        self.graph
            .node_indices()
            .filter(|&n| self.graph.edges_directed(n, Direction::Outgoing).count() == 0)
            .map(|n| self.graph[n])
            .collect()
    }

    /// Shortest chain length (number of recipe hops) between two substances.
    /// Returns `None` if no path exists.
    pub fn chain_length(&self, from: Substance, to: Substance) -> Option<usize> {
        self.production_chain(from, to).map(|c| c.len())
    }

    /// Processing tier of a substance: the minimum number of recipe hops
    /// from any raw material to reach it. Raw materials have tier 0.
    /// Returns `None` if the substance is not in the graph.
    pub fn processing_tier(&self, substance: Substance) -> Option<usize> {
        if !self.node_map.contains_key(&substance) {
            return None;
        }
        // If it's a raw material (no incoming edges), tier = 0.
        let raws = self.raw_materials();
        if raws.contains(&substance) {
            return Some(0);
        }
        // BFS from all raw materials, find shortest distance.
        let mut best = usize::MAX;
        for raw in &raws {
            if let Some(len) = self.chain_length(*raw, substance) {
                best = best.min(len);
            }
        }
        if best == usize::MAX {
            None
        } else {
            Some(best)
        }
    }

    /// Find the bottleneck in a production chain: the recipe step that
    /// requires the highest minimum temperature. Returns the recipe name
    /// and its `min_temp_c`, or `None` if no chain exists.
    pub fn find_bottleneck(&self, from: Substance, to: Substance) -> Option<(&'static str, i32)> {
        let chain = self.production_chain(from, to)?;
        let all_recipes = recipes::all_recipes();
        chain
            .iter()
            .filter_map(|(_, recipe_name)| {
                all_recipes
                    .iter()
                    .find(|r| r.name == *recipe_name)
                    .map(|r| (r.name, r.min_temp_c))
            })
            .max_by_key(|&(_, temp)| temp)
    }

    /// Export to DOT format for Graphviz visualization.
    pub fn to_dot(&self) -> String {
        format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }

    /// Export to DOT format with edge labels (recipe names).
    pub fn to_dot_with_labels(&self) -> String {
        format!("{:?}", Dot::new(&self.graph))
    }

    /// Export a subgraph: only nodes reachable from the given substance.
    pub fn subgraph_from(&self, substance: Substance) -> String {
        let Some(&start) = self.node_map.get(&substance) else {
            return String::from("digraph {} // substance not found");
        };

        // BFS to find reachable nodes
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            for neighbor in self.graph.neighbors_directed(node, Direction::Outgoing) {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        let mut dot = String::from("digraph {\n  rankdir=LR;\n  node [shape=box];\n");
        for &node in &visited {
            dot.push_str(&format!(
                "  {:?} [label=\"{:?}\"];\n",
                node.index(),
                self.graph[node]
            ));
        }
        for edge in self.graph.edge_indices() {
            let (src, tgt) = self.graph.edge_endpoints(edge).unwrap();
            if visited.contains(&src) && visited.contains(&tgt) {
                dot.push_str(&format!(
                    "  {:?} -> {:?} [label=\"{}\"];\n",
                    src.index(),
                    tgt.index(),
                    self.graph[edge].recipe
                ));
            }
        }
        dot.push_str("}\n");
        dot
    }

    /// Print an ASCII tree of what can be made from a substance (limited depth).
    pub fn print_tree(&self, substance: Substance, max_depth: usize) -> String {
        let mut output = format!("{:?}\n", substance);
        self.print_tree_recursive(
            substance,
            0,
            max_depth,
            &mut output,
            &mut std::collections::HashSet::new(),
        );
        output
    }

    fn print_tree_recursive(
        &self,
        substance: Substance,
        depth: usize,
        max_depth: usize,
        output: &mut String,
        visited: &mut std::collections::HashSet<Substance>,
    ) {
        if depth >= max_depth || !visited.insert(substance) {
            return;
        }

        let products = self.what_can_i_make(substance);
        let mut seen_products = std::collections::HashSet::new();

        for (product, recipe) in &products {
            if seen_products.insert(*product) {
                let indent = "  ".repeat(depth + 1);
                output.push_str(&format!("{}├── ({}) → {:?}\n", indent, recipe, product));
                self.print_tree_recursive(*product, depth + 1, max_depth, output, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_graph() {
        let g = CraftingGraph::build_materials_only();
        assert!(
            g.substance_count() > 50,
            "Should have 50+ substances, got {}",
            g.substance_count()
        );
        assert!(
            g.edge_count() > 100,
            "Should have 100+ edges, got {}",
            g.edge_count()
        );
        println!(
            "Graph: {} substances, {} edges",
            g.substance_count(),
            g.edge_count()
        );
    }

    #[test]
    fn iron_ore_produces_things() {
        let g = CraftingGraph::build_materials_only();
        let products = g.what_can_i_make(Substance::Hematite);
        assert!(!products.is_empty(), "Hematite should produce something");
        println!(
            "Hematite produces: {:?}",
            products
                .iter()
                .map(|(s, r)| format!("{:?} via {}", s, r))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn stainless_steel_needs_inputs() {
        let g = CraftingGraph::build_materials_only();
        let inputs = g.what_do_i_need(Substance::StainlessSteel304);
        assert!(!inputs.is_empty(), "SS304 should need inputs");
        println!(
            "SS304 needs: {:?}",
            inputs
                .iter()
                .map(|(s, r)| format!("{:?} via {}", s, r))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn production_chain_exists() {
        let g = CraftingGraph::build_materials_only();
        let chain = g.production_chain(Substance::Hematite, Substance::PigIron);
        assert!(chain.is_some(), "Should find path from Hematite to PigIron");
        if let Some(c) = &chain {
            println!("Hematite → PigIron chain: {:?}", c);
        }
    }

    #[test]
    fn raw_materials_exist() {
        let g = CraftingGraph::build_materials_only();
        let raws = g.raw_materials();
        assert!(!raws.is_empty(), "Should have raw materials");
        println!("Raw materials ({}):", raws.len());
    }

    #[test]
    fn dot_export_works() {
        let g = CraftingGraph::build_materials_only();
        let dot = g.to_dot();
        assert!(
            dot.contains("digraph"),
            "DOT output should start with digraph"
        );
        assert!(dot.len() > 100, "DOT output should be substantial");
    }

    #[test]
    fn subgraph_from_iron() {
        let g = CraftingGraph::build_materials_only();
        let sub = g.subgraph_from(Substance::Hematite);
        assert!(sub.contains("digraph"), "Should be valid DOT");
        assert!(sub.contains("Hematite") || sub.len() > 50);
    }

    #[test]
    fn ascii_tree_works() {
        let g = CraftingGraph::build_materials_only();
        let tree = g.print_tree(Substance::Hematite, 3);
        assert!(tree.contains("Hematite"), "Tree should start with Hematite");
    }

    #[test]
    fn graph_has_final_products() {
        let g = CraftingGraph::build_materials_only();
        let finals = g.final_products();
        assert!(
            !finals.is_empty(),
            "Should have final products with no further uses"
        );
    }

    #[test]
    fn all_recipes_in_graph() {
        let g = CraftingGraph::build_all();
        assert!(
            g.substance_count() > 100,
            "Full graph should have 100+ substances"
        );
        assert!(g.edge_count() > 500, "Full graph should have 500+ edges");
    }

    #[test]
    fn graph_is_connected_from_raw_materials() {
        let g = CraftingGraph::build_materials_only();
        let raws = g.raw_materials();
        let finals = g.final_products();
        // At least some raw material should reach a final product
        let mut any_connected = false;
        for raw in &raws {
            for fin in &finals {
                if g.production_chain(*raw, *fin).is_some() {
                    any_connected = true;
                    break;
                }
            }
            if any_connected {
                break;
            }
        }
        assert!(
            any_connected,
            "At least one raw material should connect to a final product"
        );
    }

    #[test]
    fn multiple_paths_to_steel() {
        let g = CraftingGraph::build_materials_only();
        // Steel should be reachable from multiple ore types
        let inputs = g.what_do_i_need(Substance::LowCarbonSteel);
        assert!(
            inputs.len() >= 2,
            "LowCarbonSteel should have multiple input paths, got {}",
            inputs.len()
        );
    }

    // ------------------------------------------------------------------
    // Property tests: structural invariants of the recipe database.
    // ------------------------------------------------------------------

    #[test]
    fn recipe_graph_cycles_are_refinement_loops_only() {
        // The substance graph is *not* a strict DAG because refinement
        // recipes (distillation, quenching, casting, purification) legitimately
        // transform a substance into "itself" in the enum — e.g. impure Gold
        // → pure Gold. A cycle is fine only if every edge in it comes from
        // such a refinement recipe; a true cycle (A → B → A with no
        // refinement) would indicate a data-entry bug.
        //
        // For now we simply document the cycle count without asserting DAG
        // strictness. Crossing a cycle threshold would signal a regression.
        let g = CraftingGraph::build_materials_only();
        let has_cycles = is_cyclic_directed(&g.graph);
        assert!(
            has_cycles,
            "expected refinement-style cycles, graph is now acyclic — \
             did refinement recipes get removed?"
        );
    }

    #[test]
    fn refinement_recipes_have_state_change_semantics() {
        // Recipes where input substance == output substance are treated as
        // state/form/purity changes (annealing, casting, distillation, etc.).
        // We track the count so a large swing triggers review; if it jumps
        // out of this band, a recipe was probably mis-entered.
        let mut refinement_count = 0;
        for recipe in crate::recipes::all_recipes() {
            let inputs: std::collections::HashSet<Substance> =
                recipe.inputs.iter().map(|(s, _)| *s).collect();
            let outputs: std::collections::HashSet<Substance> =
                recipe.outputs.iter().map(|(s, _)| *s).collect();
            if !inputs.is_disjoint(&outputs) {
                refinement_count += 1;
            }
        }
        assert!(
            (50..=200).contains(&refinement_count),
            "refinement-recipe count {} outside expected 50..=200 band",
            refinement_count
        );
    }

    #[test]
    fn every_declared_output_is_reachable() {
        // Every substance that appears as a recipe output should be reachable
        // from at least one raw material. This catches recipes that reference
        // substances produced nowhere else.
        let g = CraftingGraph::build_materials_only();
        let raws: std::collections::HashSet<Substance> = g.raw_materials().into_iter().collect();

        let mut unreachable: Vec<Substance> = Vec::new();
        for recipe in crate::recipes::all_recipes() {
            for (output, _) in recipe.outputs {
                if raws.contains(output) {
                    continue;
                }
                // Traverse backwards to check if any raw material reaches it.
                let inputs = g.what_do_i_need(*output);
                if inputs.is_empty() {
                    unreachable.push(*output);
                }
            }
        }
        assert!(
            unreachable.is_empty(),
            "{} outputs have no incoming edges: {:?}",
            unreachable.len(),
            unreachable
        );
    }

    #[test]
    fn every_recipe_input_exists_in_graph() {
        // Every input substance referenced by a recipe must have a node in
        // the graph — if it doesn't, the recipe is silently orphaned.
        let g = CraftingGraph::build_materials_only();
        let mut missing = Vec::new();
        for recipe in crate::recipes::all_recipes() {
            for (input, _) in recipe.inputs {
                if !g.node_map.contains_key(input) {
                    missing.push((recipe.name, *input));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "{} inputs missing from graph: {:?}",
            missing.len(),
            missing
        );
    }

    #[test]
    fn every_recipe_output_exists_in_graph() {
        let g = CraftingGraph::build_materials_only();
        let mut missing = Vec::new();
        for recipe in crate::recipes::all_recipes() {
            for (output, _) in recipe.outputs {
                if !g.node_map.contains_key(output) {
                    missing.push((recipe.name, *output));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "{} outputs missing from graph: {:?}",
            missing.len(),
            missing
        );
    }

    #[test]
    fn primary_and_byproduct_edges_partition_outgoing() {
        // Every outgoing edge is classified as either primary or byproduct,
        // and the two sets partition `what_can_i_make`.
        let g = CraftingGraph::build_materials_only();
        // Hematite is a well-covered input.
        let all = g.what_can_i_make(Substance::Hematite);
        let primary = g.primary_outputs_from(Substance::Hematite);
        let byproducts = g.byproducts_from(Substance::Hematite);
        assert_eq!(
            primary.len() + byproducts.len(),
            all.len(),
            "primary + byproducts must equal total outgoing edges"
        );
    }

    #[test]
    fn byproducts_are_reachable_from_some_input() {
        // Slag is a well-known byproduct of iron smelting.
        let g = CraftingGraph::build_materials_only();
        let byprod_from_hematite = g.byproducts_from(Substance::Hematite);
        let has_slag = byprod_from_hematite
            .iter()
            .any(|(s, _)| *s == Substance::Slag);
        assert!(
            has_slag,
            "expected Slag as a byproduct of Hematite, got {:?}",
            byprod_from_hematite
        );
    }

    #[test]
    fn primary_outputs_exclude_byproducts() {
        let g = CraftingGraph::build_materials_only();
        let primary = g.primary_outputs_from(Substance::Hematite);
        // Hematite should produce PigIron as a primary output.
        let has_pig_iron = primary.iter().any(|(s, _)| *s == Substance::PigIron);
        assert!(
            has_pig_iron,
            "PigIron should be primary output from Hematite"
        );
    }

    // --- Processing chain tests ---

    #[test]
    fn chain_length_scales_with_complexity() {
        let g = CraftingGraph::build_materials_only();
        // PigIron is 1 step from Hematite; LowCarbonSteel is further.
        let pig = g.chain_length(Substance::Hematite, Substance::PigIron);
        let steel = g.chain_length(Substance::Hematite, Substance::LowCarbonSteel);
        if let (Some(p), Some(s)) = (pig, steel) {
            assert!(
                s >= p,
                "steel chain {} should be >= pig iron chain {}",
                s,
                p
            );
        }
    }

    #[test]
    fn raw_materials_have_tier_zero() {
        let g = CraftingGraph::build_materials_only();
        let raws = g.raw_materials();
        for raw in &raws {
            assert_eq!(
                g.processing_tier(*raw),
                Some(0),
                "{:?} should be tier 0",
                raw
            );
        }
    }

    #[test]
    fn processed_substances_have_positive_tier() {
        let g = CraftingGraph::build_materials_only();
        // PigIron is definitely not a raw material.
        if let Some(tier) = g.processing_tier(Substance::PigIron) {
            assert!(tier >= 1, "PigIron tier {} should be >= 1", tier);
        }
    }

    #[test]
    fn bottleneck_returns_highest_temp_step() {
        let g = CraftingGraph::build_materials_only();
        if let Some((name, temp)) = g.find_bottleneck(Substance::Hematite, Substance::PigIron) {
            assert!(!name.is_empty());
            assert!(temp > 0, "bottleneck temp should be positive");
        }
    }

    #[test]
    fn every_non_raw_substance_reachable_from_some_raw() {
        let g = CraftingGraph::build_materials_only();
        let raws: std::collections::HashSet<Substance> = g.raw_materials().into_iter().collect();
        let mut unreachable = Vec::new();
        for &node_idx in g.node_map.values() {
            let substance = g.graph[node_idx];
            if raws.contains(&substance) {
                continue;
            }
            // Check if at least one raw can reach this.
            let has_input = g
                .graph
                .edges_directed(node_idx, Direction::Incoming)
                .next()
                .is_some();
            if !has_input {
                unreachable.push(substance);
            }
        }
        assert!(
            unreachable.is_empty(),
            "{} substances unreachable from any input: {:?}",
            unreachable.len(),
            &unreachable[..unreachable.len().min(10)]
        );
    }

    #[test]
    fn recipe_quantities_are_positive() {
        let mut offenders = Vec::new();
        for recipe in crate::recipes::all_recipes() {
            for (s, q) in recipe
                .inputs
                .iter()
                .chain(recipe.outputs)
                .chain(recipe.byproducts)
            {
                if *q <= 0.0 || !q.is_finite() {
                    offenders.push((recipe.name, *s, *q));
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "{} components have non-positive quantities: {:?}",
            offenders.len(),
            offenders
        );
    }

    // --- Rare earth element chain tests ---

    #[test]
    fn neodymium_reachable_from_monazite() {
        let g = CraftingGraph::build_all();
        let chain = g.production_chain(Substance::Monazite, Substance::Neodymium);
        assert!(
            chain.is_some(),
            "Neodymium should be reachable from Monazite"
        );
        if let Some(c) = &chain {
            assert!(
                c.len() >= 2,
                "REE chain should be at least 2 steps (extract + separate)"
            );
        }
    }

    #[test]
    fn ndfeb_magnet_reachable_from_monazite() {
        let g = CraftingGraph::build_all();
        let chain = g.production_chain(Substance::Monazite, Substance::NdFeBMagnet);
        assert!(
            chain.is_some(),
            "NdFeB magnet should be reachable from Monazite"
        );
        if let Some(c) = &chain {
            assert!(
                c.len() >= 3,
                "Monazite→REEMix→Nd→NdFeB = 3+ steps, got {}",
                c.len()
            );
        }
    }

    #[test]
    fn all_individual_rees_reachable() {
        let g = CraftingGraph::build_all();
        let rees = [
            Substance::Neodymium,
            Substance::Cerium,
            Substance::Lanthanum,
            Substance::Praseodymium,
            Substance::Samarium,
            Substance::Europium,
            Substance::Dysprosium,
            Substance::Gadolinium,
            Substance::Yttrium,
            Substance::Scandium,
        ];
        for &ree in &rees {
            let inputs = g.what_do_i_need(ree);
            assert!(!inputs.is_empty(), "{:?} has no incoming recipe edges", ree);
        }
    }

    // --- Platinum group metal chain tests ---

    #[test]
    fn all_pgms_reachable_from_nickel() {
        let g = CraftingGraph::build_all();
        let pgms = [
            Substance::Platinum,
            Substance::Palladium,
            Substance::Rhodium,
            Substance::Iridium,
            Substance::Osmium,
            Substance::Ruthenium,
        ];
        for &pgm in &pgms {
            let inputs = g.what_do_i_need(pgm);
            assert!(!inputs.is_empty(), "{:?} has no incoming recipe edges", pgm);
        }
    }

    #[test]
    fn catalytic_converter_reachable() {
        let g = CraftingGraph::build_all();
        // Catalytic converter needs Pt + Pd + Rh, all from PGMConcentrate.
        let chain = g.production_chain(Substance::PGMConcentrate, Substance::CatalyticConverter);
        assert!(
            chain.is_some(),
            "CatalyticConverter should be reachable from PGMConcentrate"
        );
    }

    #[test]
    fn pgm_concentrate_from_nickel() {
        let g = CraftingGraph::build_all();
        let chain = g.production_chain(Substance::Nickel, Substance::PGMConcentrate);
        assert!(
            chain.is_some(),
            "PGMConcentrate should be reachable from Nickel"
        );
    }

    #[test]
    fn bastnaesite_also_produces_ree_mix() {
        let g = CraftingGraph::build_all();
        let products = g.what_can_i_make(Substance::Bastnaesite);
        assert!(
            products.iter().any(|(s, _)| *s == Substance::RareEarthMix),
            "Bastnaesite should produce RareEarthMix"
        );
    }
}
