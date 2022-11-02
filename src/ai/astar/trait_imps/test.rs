// =============================================================================
//! - Unit tests for the A* algorithm trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-02
//! - Rust since: 2022-11-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::{
  ai::astar::structures::{GradientCartographer, Rectangle},
  math::geom::structures::Point2DD,
};

#[test]
fn test_estimate_to_goal() {
  let test_subject_gradient_cartographer = GradientCartographer {
    directions: 8,
    goal_node: &Point2DD::default(),
    init_step_size: 1.0,
    node_factory: &Point2DD::default(),
    space_tester: &Rectangle::default(),
    start_node: &Point2DD::default(),
  };
  // TODO: left off here
}
