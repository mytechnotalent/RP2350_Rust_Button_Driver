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
//! RP2350 Blink Configuration Constants.
//!
//! BRIEF:
//! Defines configuration constants for LED blink timing.
//! Contains delay intervals and GPIO pin configuration.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: November 28, 2025
//! UPDATE DATE: December 4, 2025

/// Default LED blink delay in milliseconds.
///
/// # Details
/// Configures the delay between LED state transitions.
/// Used for both ON and OFF durations.
///
/// # Value
/// 500 milliseconds
pub const BLINK_DELAY_MS: u64 = 500;

/// Minimum allowed blink delay in milliseconds.
///
/// # Details
/// Prevents excessively fast blinking which may cause issues.
///
/// # Value
/// 10 milliseconds
#[allow(dead_code)]
pub const MIN_BLINK_DELAY_MS: u64 = 10;

/// Maximum allowed blink delay in milliseconds.
///
/// # Details
/// Prevents excessively slow blinking for practical use.
///
/// # Value
/// 10000 milliseconds (10 seconds)
#[allow(dead_code)]
pub const MAX_BLINK_DELAY_MS: u64 = 10000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink_delay_default() {
        assert_eq!(BLINK_DELAY_MS, 500);
    }

    #[test]
    fn test_min_delay_less_than_default() {
        assert!(MIN_BLINK_DELAY_MS < BLINK_DELAY_MS);
    }

    #[test]
    fn test_max_delay_greater_than_default() {
        assert!(MAX_BLINK_DELAY_MS > BLINK_DELAY_MS);
    }

    #[test]
    fn test_delay_range_valid() {
        assert!(MIN_BLINK_DELAY_MS < MAX_BLINK_DELAY_MS);
    }
}
