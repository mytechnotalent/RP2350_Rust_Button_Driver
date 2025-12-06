/*
 * @file button.rs
 * @brief Button input with debouncing
 * @author Kevin Thomas
 * @date 2025
 *
 * MIT License
 *
 * Copyright (c) 2025 Kevin Thomas
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! FILE: button.rs
//!
//! DESCRIPTION:
//! RP2350 Button Input with Debouncing.
//!
//! BRIEF:
//! Implements button state tracking with debounce logic.
//! Button is active-low (tied to GND when pressed).
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 5, 2025
//! UPDATE DATE: December 5, 2025

use crate::config::DEBOUNCE_COUNT;

/// Button controller with debouncing.
///
/// # Details
/// Maintains button state with software debouncing.
/// Uses sample-based debouncing for reliable detection.
///
/// # Fields
/// * `pressed` - Current debounced button state (true = pressed)
/// * `raw_pressed` - Current raw (unfiltered) state
/// * `debounce_count` - Current debounce counter
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct ButtonController {
    pressed: bool,
    raw_pressed: bool,
    debounce_count: u32,
}

impl Default for ButtonController {
    /// Returns default ButtonController instance.
    ///
    /// # Details
    /// Delegates to new() for initialization.
    ///
    /// # Returns
    /// * `Self` - New ButtonController with default values
    #[allow(dead_code)]
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonController {
    /// Creates new button controller.
    ///
    /// # Details
    /// Initializes controller with button released state.
    ///
    /// # Returns
    /// * `Self` - New ButtonController instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            pressed: false,
            raw_pressed: false,
            debounce_count: 0,
        }
    }

    /// Updates button state with new GPIO sample.
    ///
    /// # Details
    /// Processes raw GPIO input through debounce filter.
    /// Active-low: false (low GPIO) means pressed.
    ///
    /// # Arguments
    /// * `gpio_high` - true if GPIO high (released), false if low (pressed)
    #[allow(dead_code)]
    pub fn update(&mut self, gpio_high: bool) {
        let new_raw = !gpio_high;
        if new_raw == self.raw_pressed {
            if self.debounce_count < DEBOUNCE_COUNT {
                self.debounce_count += 1;
            }
        } else {
            self.raw_pressed = new_raw;
            self.debounce_count = 0;
        }
        if self.debounce_count >= DEBOUNCE_COUNT {
            self.pressed = self.raw_pressed;
        }
    }

    /// Returns true if button is pressed.
    ///
    /// # Details
    /// Returns debounced button state.
    ///
    /// # Returns
    /// * `bool` - true if button is pressed
    #[allow(dead_code)]
    pub fn is_pressed(&self) -> bool {
        self.pressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ButtonController Construction Tests ====================

    #[test]
    fn test_new_controller_not_pressed() {
        let ctrl = ButtonController::new();
        assert!(!ctrl.is_pressed());
    }

    #[test]
    fn test_default_equals_new() {
        let default = ButtonController::default();
        let new = ButtonController::new();
        assert_eq!(default.is_pressed(), new.is_pressed());
    }

    // ==================== Debounce Logic Tests ====================

    #[test]
    fn test_no_change_when_gpio_stays_high() {
        let mut ctrl = ButtonController::new();
        for _ in 0..10 {
            ctrl.update(true);
        }
        assert!(!ctrl.is_pressed());
    }

    #[test]
    fn test_pressed_after_threshold() {
        let mut ctrl = ButtonController::new();
        for _ in 0..=DEBOUNCE_COUNT {
            ctrl.update(false);
        }
        assert!(ctrl.is_pressed());
    }

    #[test]
    fn test_released_after_threshold() {
        let mut ctrl = ButtonController::new();
        for _ in 0..=DEBOUNCE_COUNT {
            ctrl.update(false);
        }
        for _ in 0..=DEBOUNCE_COUNT {
            ctrl.update(true);
        }
        assert!(!ctrl.is_pressed());
    }

    #[test]
    fn test_debounce_resets_on_bounce() {
        let mut ctrl = ButtonController::new();
        for _ in 0..(DEBOUNCE_COUNT - 1) {
            ctrl.update(false);
        }
        ctrl.update(true);
        for _ in 0..(DEBOUNCE_COUNT - 1) {
            ctrl.update(false);
        }
        assert!(!ctrl.is_pressed());
    }

    // ==================== Bounce Rejection Tests ====================

    #[test]
    fn test_rapid_bouncing_rejected() {
        let mut ctrl = ButtonController::new();
        for _ in 0..10 {
            ctrl.update(false);
            ctrl.update(true);
        }
        assert!(!ctrl.is_pressed());
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_exactly_at_threshold() {
        let mut ctrl = ButtonController::new();
        for _ in 0..DEBOUNCE_COUNT {
            ctrl.update(false);
        }
        assert!(!ctrl.is_pressed());
        ctrl.update(false);
        assert!(ctrl.is_pressed());
    }

    #[test]
    fn test_one_sample_before_threshold() {
        let mut ctrl = ButtonController::new();
        for _ in 0..(DEBOUNCE_COUNT) {
            ctrl.update(false);
        }
        assert!(!ctrl.is_pressed());
    }

    #[test]
    fn test_state_persists_after_achieving_threshold() {
        let mut ctrl = ButtonController::new();
        for _ in 0..=DEBOUNCE_COUNT {
            ctrl.update(false);
        }
        assert!(ctrl.is_pressed());
        for _ in 0..5 {
            ctrl.update(false);
            assert!(ctrl.is_pressed());
        }
    }

    // ==================== Trait Implementation Tests ====================

    #[test]
    fn test_clone() {
        let ctrl1 = ButtonController::new();
        let ctrl2 = ctrl1;
        assert_eq!(ctrl1.is_pressed(), ctrl2.is_pressed());
    }

    #[test]
    fn test_partial_eq() {
        let ctrl1 = ButtonController::new();
        let ctrl2 = ButtonController::new();
        assert_eq!(ctrl1, ctrl2);
    }

    #[test]
    fn test_debug_format() {
        let ctrl = ButtonController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("ButtonController"));
    }
}
