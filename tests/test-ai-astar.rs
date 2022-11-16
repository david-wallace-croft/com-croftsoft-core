// =============================================================================
//! - A* algorithm integration tests
//!
//! # Metadata
//! - Copyright: &copy; 2002 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-16
//! - Rust since: 2022-11-13
//! - Java version: 2003-05-09
//! - Java since: 2002-04-21
//!
//! # History
//! - Adapted from the class in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.ai.astar.AStarTest
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use std::collections::HashSet;
use std::hash::Hash;

use com_croftsoft_core::ai::astar::structures::AStar;
use com_croftsoft_core::ai::astar::traits::Cartographer;

type Point = (i64, i64);

const LOOP_COUNT_MAX: usize = 1_000;

const DISTANCE_FROM_ORIGIN: usize = 10;
const X_MAX: i64 = DISTANCE_FROM_ORIGIN as i64;
const X_MIN: i64 = -(DISTANCE_FROM_ORIGIN as i64);
const Y_MAX: i64 = DISTANCE_FROM_ORIGIN as i64;
const Y_MIN: i64 = -(DISTANCE_FROM_ORIGIN as i64);

const GOAL_3: Point = (-3, 0);
const GOAL_4: Point = (4, 0);
const GOAL_5: Point = (5, 0);
const TELEPORT_MAX: Point = (X_MAX, Y_MAX);
const TELEPORT_MIN: Point = (X_MIN, Y_MIN);

// Finds its way around a wall
const BLOCKED_OBSTACLE: [Point; 3] = [
  (1, 1),
  (1, 0),
  (1, -1),
];

// Start enclosed by walls
const BLOCKED_ENCLOSED_START: [Point; 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

// Goal enclosed by walls
const BLOCKED_ENCLOSED_GOAL: [Point; 8] = [
  (4, -1),
  (4, 0),
  (4, 1),
  (5, -1),
  (5, 1),
  (6, -1),
  (6, 0),
  (6, 1),
];

// Goal enclosed by walls but teleport jump available
const BLOCKED_ENCLOSED_GOAL_TELEPORT: [Point; 8] = [
  (3, -1),
  (3, 0),
  (3, 1),
  (4, -1),
  (4, 1),
  (5, -1),
  (5, 0),
  (5, 1),
];

// Goal clear on right but teleport jump on left closer
const BLOCKED_NO_OBSTACLES: [Point; 0] = [];

pub struct AStarTest<N: Eq + Hash> {
  pub blocked_set: HashSet<N>,
  pub goal_point: N,
  pub jump_point_option: Option<N>,
}

impl Cartographer<Point> for AStarTest<Point> {
  fn estimate_cost_to_goal(
    &self,
    node: &Point,
  ) -> f64 {
    self.get_cost_to_adjacent_node(node, &self.goal_point)
  }

  fn get_adjacent_nodes(
    &self,
    node: &Point,
  ) -> Vec<Point> {
    let x: i64 = node.0;
    let y: i64 = node.1;
    let mut list = Vec::new();
    if self.jump_point_option.is_some()
      && self.jump_point_option.unwrap() == *node
    {
      list.push(self.goal_point);
      return list;
    }
    for offset_x in -1..2 {
      for offset_y in -1..2 {
        if offset_x == 0 && offset_y == 0 {
          continue;
        }
        let new_x = x + offset_x;
        let new_y = y + offset_y;
        if new_x < X_MIN || new_y < Y_MIN || new_x > X_MAX || new_y > Y_MAX {
          continue;
        }
        let new_point: Point = (new_x, new_y);
        if !self.blocked_set.contains(&new_point) {
          list.push(new_point);
        }
      }
    }
    list
  }

  fn get_cost_to_adjacent_node(
    &self,
    from_node: &Point,
    to_node: &Point,
  ) -> f64 {
    if self.jump_point_option.is_some()
      && self.jump_point_option.unwrap() == *from_node
    {
      return 0.0;
    }
    let delta0 = from_node.0 - to_node.0;
    let delta1 = from_node.1 - to_node.0;
    ((delta0.pow(2) + delta1.pow(2)) as f64).sqrt()
  }

  fn is_goal_node(
    &self,
    node: &Point,
  ) -> bool {
    *node == self.goal_point
  }
}

impl AStarTest<Point> {
  pub fn new(
    blocked_points: &[Point],
    goal_point: Point,
    jump_point_option: Option<Point>,
  ) -> Self {
    let mut blocked_set = HashSet::new();
    for blocked_point in blocked_points {
      blocked_set.insert(*blocked_point);
    }
    AStarTest {
      blocked_set,
      goal_point,
      jump_point_option,
    }
  }
}

#[test]
fn test_ai_astar_enclosed_goal() {
  let astar_test =
    AStarTest::<Point>::new(&BLOCKED_ENCLOSED_GOAL, GOAL_5, None);
  let mut astar = AStar::<AStarTest<Point>, Point>::new(astar_test);
  astar.reset((0, 0));
  let mut loop_count = 0;
  while loop_count < LOOP_COUNT_MAX && astar.loop_once() {
    loop_count += 1;
  }
  assert_ne!(loop_count, LOOP_COUNT_MAX);
  assert!(!astar.is_goal_found());
}

#[test]
fn test_ai_astar_enclosed_goal_teleport() {
  let astar_test = AStarTest::<Point>::new(
    &BLOCKED_ENCLOSED_GOAL_TELEPORT,
    GOAL_4,
    Some(TELEPORT_MIN),
  );
  let mut astar = AStar::<AStarTest<Point>, Point>::new(astar_test);
  astar.reset((0, 0));
  let mut loop_count = 0;
  while loop_count < LOOP_COUNT_MAX && astar.loop_once() {
    loop_count += 1;
  }
  assert_ne!(loop_count, LOOP_COUNT_MAX);
  assert!(astar.is_goal_found());
  let path: Vec<Point> = astar.get_path();
  assert_eq!(path.len(), DISTANCE_FROM_ORIGIN + 1);
  assert_eq!(path[DISTANCE_FROM_ORIGIN], GOAL_4);
}

#[test]
fn test_ai_astar_enclosed_start() {
  let astar_test =
    AStarTest::<Point>::new(&BLOCKED_ENCLOSED_START, GOAL_4, None);
  let mut astar = AStar::<AStarTest<Point>, Point>::new(astar_test);
  astar.reset((0, 0));
  let mut loop_count = 0;
  while loop_count < LOOP_COUNT_MAX && astar.loop_once() {
    loop_count += 1;
  }
  assert_ne!(loop_count, LOOP_COUNT_MAX);
  assert!(!astar.is_goal_found());
}

#[test]
fn test_ai_astar_obstacle() {
  let astar_test = AStarTest::<Point>::new(&BLOCKED_OBSTACLE, GOAL_4, None);
  let mut astar = AStar::<AStarTest<Point>, Point>::new(astar_test);
  astar.reset((0, 0));
  let mut loop_count = 0;
  while loop_count < LOOP_COUNT_MAX && astar.loop_once() {
    loop_count += 1;
  }
  assert_ne!(loop_count, LOOP_COUNT_MAX);
  assert!(astar.is_goal_found());
  let path: Vec<Point> = astar.get_path();
  assert_eq!(path.len(), 5);
  assert_eq!(path[4], GOAL_4);
}

#[test]
fn test_ai_astar_teleport_nearby() {
  let astar_test =
    AStarTest::<Point>::new(&BLOCKED_NO_OBSTACLES, GOAL_3, Some(TELEPORT_MAX));
  let mut astar = AStar::<AStarTest<Point>, Point>::new(astar_test);
  astar.reset((0, 0));
  let mut loop_count = 0;
  while loop_count < LOOP_COUNT_MAX && astar.loop_once() {
    loop_count += 1;
  }
  assert_ne!(loop_count, LOOP_COUNT_MAX);
  assert!(astar.is_goal_found());
  let path: Vec<Point> = astar.get_path();
  assert_eq!(path.len(), 3);
  assert_eq!(path[2], GOAL_3);
}
