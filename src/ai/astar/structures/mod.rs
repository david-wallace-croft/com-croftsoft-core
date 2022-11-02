// =============================================================================
//! - Structures for the A* algorithm
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-02
//! - Rust since: 2022-10-22
//! - Java version: 2003-05-10
//! - Java since: 2002-04-21
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.GridCartographer
//!   - com.croftsoft.core.ai.astar.NodeInfo
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::traits::{Cartographer, NodeFactory, SpaceTester};
use crate::math::geom::traits::PointXY;
use core::hash::Hash;
use std::collections::HashMap;

pub struct AStar<'c, C: Cartographer<N>, N: Eq + Hash + PointXY> {
  pub best_node_info: NodeInfo<N>,
  pub best_total_cost: f64,
  pub cartographer: &'c C,
  pub goal_node_info_option: Option<NodeInfo<N>>,
  pub list_empty: bool,
  pub node_to_node_info_map: HashMap<N, NodeInfo<N>>,
  pub node_to_parent_node_info_map: HashMap<N, NodeInfo<N>>,
  pub open_node_info_sorted_list: Vec<NodeInfo<N>>,
}

/// Gradient cartographer for continuous space.
/// The adjacent nodes are spaced farther apart as you move away from the
/// starting point.
pub struct GradientCartographer<
  'f,
  'n,
  's,
  F: NodeFactory<N>,
  N: PointXY,
  S: SpaceTester<N>,
> {
  pub directions: u64,
  pub goal_node: &'n N,
  pub init_step_size: f64,
  pub node_factory: &'f F,
  pub space_tester: &'s S,
  pub start_node: &'n N,
}

/// Grid cartographer for continuous space.
/// The nodes are spaced equally apart in the eight cardinal directions.
pub struct GridCartographer<
  'f,
  'n,
  's,
  F: NodeFactory<N>,
  N: PointXY,
  S: SpaceTester<N>,
> {
  pub goal_node: &'n N,
  pub node_factory: &'f F,
  pub space_tester: &'s S,
  pub step_size: f64,
}

#[derive(Clone, Copy, Debug)]
/// A* algorithm node information
pub struct NodeInfo<N: PointXY> {
  pub cost_from_start: f64,
  pub node: N,
  // pub parent_node_info_option: Option<&'i NodeInfo<'i, N>>,
  pub total_cost: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
  pub x_max: f64,
  pub x_min: f64,
  pub y_max: f64,
  pub y_min: f64,
}
