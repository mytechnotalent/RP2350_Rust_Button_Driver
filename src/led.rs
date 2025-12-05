/*
 * @file led.rs
 * @brief LED state management
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

//! FILE: led.rs
//!
//! DESCRIPTION:
//! LED State Management for RP2350.
//!
//! BRIEF:
//! Provides LED state enumeration and utility functions.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 5, 2025
//! UPDATE DATE: December 5, 2025

/// LED state enumeration.
///
/// # Details
/// Represents the current state of the LED.
/// Used for state tracking and transitions.
///
/// # Variants
/// * `On` - LED is currently on (high)
/// * `Off` - LED is currently off (low)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedState {
    On,
    Off,
}

/// Converts LedState to boolean for GPIO control.
///
/// # Details
/// Maps On state to true (high), Off state to false (low).
///
/// # Arguments
/// * `state` - LED state to convert
///
/// # Returns
/// * `bool` - true for On, false for Off
pub fn led_state_to_level(state: LedState) -> bool {
    matches!(state, LedState::On)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== LedState Enum Tests ====================

    #[test]
    fn test_led_state_equality() {
        assert_eq!(LedState::On, LedState::On);
        assert_eq!(LedState::Off, LedState::Off);
        assert_ne!(LedState::On, LedState::Off);
    }

    #[test]
    fn test_led_state_copy() {
        let state = LedState::On;
        let copy = state;
        assert_eq!(state, copy);
    }

    #[test]
    fn test_led_state_to_level_on() {
        assert!(led_state_to_level(LedState::On));
    }

    #[test]
    fn test_led_state_to_level_off() {
        assert!(!led_state_to_level(LedState::Off));
    }
}
