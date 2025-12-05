/*
 * @file config.rs
 * @brief Application configuration constants
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

//! FILE: config.rs
//!
//! DESCRIPTION:
//! RP2350 Button Driver Configuration Constants.
//!
//! BRIEF:
//! Defines configuration constants for button debouncing and GPIO pins.
//! Contains debounce timing and GPIO pin configuration.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: November 28, 2025
//! UPDATE DATE: December 5, 2025

/// Default debounce delay in milliseconds.
///
/// # Details
/// Configures the delay between button state samples.
/// Used for software debouncing filter.
///
/// # Value
/// 5 milliseconds
pub const DEBOUNCE_DELAY_MS: u64 = 5;

/// Default debounce sample count threshold.
///
/// # Details
/// Number of consecutive stable samples required for state change.
/// Higher values provide better noise immunity but slower response.
///
/// # Value
/// 5 samples
pub const DEBOUNCE_COUNT: u32 = 5;

/// Button GPIO pin number.
///
/// # Details
/// GPIO pin connected to the button.
/// Button is active-low (tied to GND when pressed).
///
/// # Value
/// GPIO 15
#[allow(dead_code)]
pub const BUTTON_PIN: u8 = 15;

/// LED GPIO pin number.
///
/// # Details
/// GPIO pin connected to the LED.
/// LED turns on when button is pressed.
///
/// # Value
/// GPIO 16
#[allow(dead_code)]
pub const LED_PIN: u8 = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debounce_delay_default() {
        assert_eq!(DEBOUNCE_DELAY_MS, 5);
    }

    #[test]
    fn test_debounce_count_positive() {
        assert!(DEBOUNCE_COUNT > 0);
    }

    #[test]
    fn test_button_pin_valid() {
        assert_eq!(BUTTON_PIN, 15);
    }

    #[test]
    fn test_led_pin_valid() {
        assert_eq!(LED_PIN, 16);
    }

    #[test]
    fn test_button_led_pins_different() {
        assert_ne!(BUTTON_PIN, LED_PIN);
    }
}
