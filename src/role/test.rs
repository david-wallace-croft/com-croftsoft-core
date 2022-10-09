// =============================================================================
//! - Unit tests for the traits in module "role"
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-09
//! - Since: 2022-10-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::*;

#[cfg(test)]
const UNFILTERED: [u8; 5] = [
  1, 2, 3, 4, 5,
];

#[cfg(test)]
struct EvenOddFilter {
  even: bool,
  odd: bool,
}

#[cfg(test)]
impl Filter<&u8> for EvenOddFilter {
  fn is_filtrate(
    &self,
    item: &u8,
  ) -> bool {
    let is_even = item % 2 == 0;
    if is_even {
      self.even
    } else {
      self.odd
    }
  }
}

#[test]
fn test_filter() {
  let filter = EvenOddFilter {
    even: true,
    odd: false,
  };
  let filtered_sum = UNFILTERED.iter().fold(0, |sum, item| {
    sum
      + (if filter.is_filtrate(item) {
        *item
      } else {
        0
      })
  });
  assert_eq!(filtered_sum, 6);
}
