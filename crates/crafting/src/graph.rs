use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::HashMap;

use crate::recipes::substance::Substance;
use crate::recipes::types::Recipe;
use crate::recipes;
use crate::food;

/// A directed graph where nodes are Substances and edges are Recipes.
/// Edge direction: Input → Output (following material flow).
pub struct CraftingGraph {
    pub graph: DiGraph<Substance, &'static str>,
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
        *self.node_map.entry(substance).or_insert_with(|| {
            self.graph.add_node(substance)
        })
    }

    fn add_recipe(&mut self, recipe: &'static Recipe) {
        // Create output nodes
        let output_nodes: Vec<NodeIndex> = recipe.outputs.iter()
            .map(|(s, _)| self.get_or_create_node(*s))
            .collect();

        // Create edges from each input to each output
        for (input_substance, _) in recipe.inputs {
            let input_node = self.get_or_create_node(*input_substance);
            for &output_node in &output_nodes {
                // Only add edge if it doesn't already exist with this recipe name
                if !self.graph.edges_connecting(input_node, output_node)
                    .any(|e| *e.weight() == recipe.name) {
                    self.graph.add_edge(input_node, output_node, recipe.name);
                }
            }
        }

        // Also link byproducts
        for (byproduct, _) in recipe.byproducts {
            let bp_node = self.get_or_create_node(*byproduct);
            for (input_substance, _) in recipe.inputs {
                let input_node = self.get_or_create_node(*input_substance);
                if !self.graph.edges_connecting(input_node, bp_node)
                    .any(|e| *e.weight() == recipe.name) {
                    self.graph.add_edge(input_node, bp_node, recipe.name);
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
    pub fn what_can_i_make(&self, substance: Substance) -> Vec<(Substance, &'static str)> {
        let Some(&node) = self.node_map.get(&substance) else {
            return vec![];
        };
        self.graph.edges_directed(node, Direction::Outgoing)
            .map(|e| (self.graph[e.target()], *e.weight()))
            .collect()
    }

    /// What do I need to make this substance? (backward: what are its inputs?)
    pub fn what_do_i_need(&self, substance: Substance) -> Vec<(Substance, &'static str)> {
        let Some(&node) = self.node_map.get(&substance) else {
            return vec![];
        };
        self.graph.edges_directed(node, Direction::Incoming)
            .map(|e| (self.graph[e.source()], *e.weight()))
            .collect()
    }

    /// Get the full production chain from raw material to target (BFS shortest path).
    pub fn production_chain(&self, from: Substance, to: Substance) -> Option<Vec<(Substance, &'static str)>> {
        let start = *self.node_map.get(&from)?;
        let end = *self.node_map.get(&to)?;

        // BFS
        let path = petgraph::algo::astar(
            &self.graph,
            start,
            |n| n == end,
            |_| 1,
            |_| 0,
        );

        path.map(|(_, nodes)| {
            let mut chain = Vec::new();
            for window in nodes.windows(2) {
                let from_node = window[0];
                let to_node = window[1];
                let substance = self.graph[to_node];
                let recipe_name = self.graph.edges_connecting(from_node, to_node)
                    .next()
                    .map(|e| *e.weight())
                    .unwrap_or("?");
                chain.push((substance, recipe_name));
            }
            chain
        })
    }

    /// Get all raw materials (substances with no incoming edges).
    pub fn raw_materials(&self) -> Vec<Substance> {
        self.graph.node_indices()
            .filter(|&n| self.graph.edges_directed(n, Direction::Incoming).count() == 0)
            .map(|n| self.graph[n])
            .collect()
    }

    /// Get all final products (substances with no outgoing edges).
    pub fn final_products(&self) -> Vec<Substance> {
        self.graph.node_indices()
            .filter(|&n| self.graph.edges_directed(n, Direction::Outgoing).count() == 0)
            .map(|n| self.graph[n])
            .collect()
    }

    /// Export to DOT format for Graphviz visualization.
    pub fn to_dot(&self) -> String {
        format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
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
            dot.push_str(&format!("  {:?} [label=\"{:?}\"];\n", node.index(), self.graph[node]));
        }
        for edge in self.graph.edge_indices() {
            let (src, tgt) = self.graph.edge_endpoints(edge).unwrap();
            if visited.contains(&src) && visited.contains(&tgt) {
                dot.push_str(&format!("  {:?} -> {:?} [label=\"{}\"];\n",
                    src.index(), tgt.index(), self.graph[edge]));
            }
        }
        dot.push_str("}\n");
        dot
    }

    /// Print an ASCII tree of what can be made from a substance (limited depth).
    pub fn print_tree(&self, substance: Substance, max_depth: usize) -> String {
        let mut output = format!("{:?}\n", substance);
        self.print_tree_recursive(substance, 0, max_depth, &mut output, &mut std::collections::HashSet::new());
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
                let connector = if depth == 0 { "├── " } else { "├── " };
                output.push_str(&format!("{}{}({}) → {:?}\n", indent, connector, recipe, product));
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
        assert!(g.substance_count() > 50, "Should have 50+ substances, got {}", g.substance_count());
        assert!(g.edge_count() > 100, "Should have 100+ edges, got {}", g.edge_count());
        println!("Graph: {} substances, {} edges", g.substance_count(), g.edge_count());
    }

    #[test]
    fn iron_ore_produces_things() {
        let g = CraftingGraph::build_materials_only();
        let products = g.what_can_i_make(Substance::Hematite);
        assert!(!products.is_empty(), "Hematite should produce something");
        println!("Hematite produces: {:?}", products.iter().map(|(s, r)| format!("{:?} via {}", s, r)).collect::<Vec<_>>());
    }

    #[test]
    fn stainless_steel_needs_inputs() {
        let g = CraftingGraph::build_materials_only();
        let inputs = g.what_do_i_need(Substance::StainlessSteel304);
        assert!(!inputs.is_empty(), "SS304 should need inputs");
        println!("SS304 needs: {:?}", inputs.iter().map(|(s, r)| format!("{:?} via {}", s, r)).collect::<Vec<_>>());
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
        assert!(dot.contains("digraph"), "DOT output should start with digraph");
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
        println!("{}", tree);
    }
}
