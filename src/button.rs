/*
 * @file button.rs
 * @brief Button input state machine with debouncing
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
//! RP2350 Button Input State Machine with Debouncing.
//!
//! BRIEF:
//! Implements button state tracking, debounce logic, and press detection.
//! Provides testable state machine for button input functionality.
//! Button is active-low (tied to GND when pressed).
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 5, 2025
//! UPDATE DATE: December 5, 2025

use crate::config::{DEBOUNCE_COUNT, DEBOUNCE_DELAY_MS, MAX_DEBOUNCE_DELAY_MS, MIN_DEBOUNCE_DELAY_MS};

#[cfg(feature = "embassy-rp")]
use embassy_rp::gpio::{Input, Output};
#[cfg(feature = "embassy-time")]
use embassy_time::Timer;

/// Button state enumeration.
///
/// # Details
/// Represents the current physical state of the button.
/// Active-low: pressed when GPIO reads low.
///
/// # Variants
/// * `Pressed` - Button is currently pressed (GPIO low)
/// * `Released` - Button is currently released (GPIO high)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

/// Button event enumeration.
///
/// # Details
/// Represents button state transition events.
/// Used for edge detection and event handling.
///
/// # Variants
/// * `Pressed` - Button was just pressed (falling edge)
/// * `Released` - Button was just released (rising edge)
/// * `None` - No state change occurred
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonEvent {
    Pressed,
    Released,
    None,
}

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

/// Button controller with state tracking and debouncing.
///
/// # Details
/// Maintains button state with software debouncing.
/// Provides methods for state transitions and queries.
/// Uses sample-based debouncing for reliable detection.
///
/// # Fields
/// * `state` - Current debounced button state
/// * `raw_state` - Current raw (unfiltered) button state
/// * `debounce_count` - Current debounce counter
/// * `debounce_threshold` - Required stable samples for state change
/// * `debounce_delay_ms` - Delay between samples in milliseconds
/// * `press_count` - Number of button presses detected
/// * `led_state` - Current LED state controlled by button
#[derive(Debug)]
pub struct ButtonController {
    state: ButtonState,
    raw_state: ButtonState,
    debounce_count: u32,
    debounce_threshold: u32,
    debounce_delay_ms: u64,
    press_count: u64,
    led_state: LedState,
}

/// Default implementation for ButtonController.
impl Default for ButtonController {
    fn default() -> Self {
        Self::create_initial()
    }
}

/// Public methods for ButtonController
impl ButtonController {
    /// Creates initial button controller state.
    ///
    /// # Returns
    /// * `Self` - New ButtonController with initial values
    fn create_initial() -> Self {
        Self {
            state: ButtonState::Released,
            raw_state: ButtonState::Released,
            debounce_count: 0,
            debounce_threshold: DEBOUNCE_COUNT,
            debounce_delay_ms: DEBOUNCE_DELAY_MS,
            press_count: 0,
            led_state: LedState::Off,
        }
    }

    /// Creates new button controller with default settings.
    ///
    /// # Details
    /// Initializes controller with button released using Default trait.
    /// Ready to start processing button input immediately.
    ///
    /// # Returns
    /// * `Self` - New ButtonController instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates new button controller with custom debounce settings.
    ///
    /// # Details
    /// Initializes controller with specified debounce parameters.
    /// Delay is clamped to valid range.
    ///
    /// # Arguments
    /// * `debounce_delay_ms` - Desired debounce delay in milliseconds
    /// * `debounce_threshold` - Number of stable samples required
    ///
    /// # Returns
    /// * `Self` - New ButtonController with configured debouncing
    #[allow(dead_code)]
    pub fn with_debounce(debounce_delay_ms: u64, debounce_threshold: u32) -> Self {
        Self {
            state: ButtonState::Released,
            raw_state: ButtonState::Released,
            debounce_count: 0,
            debounce_threshold,
            debounce_delay_ms: clamp_debounce_delay(debounce_delay_ms),
            press_count: 0,
            led_state: LedState::Off,
        }
    }

    /// Updates button state with new raw input sample.
    ///
    /// # Details
    /// Processes raw GPIO input through debounce filter.
    /// Returns event if state transition occurred.
    /// Active-low: false (low GPIO) means pressed.
    ///
    /// # Arguments
    /// * `gpio_level` - Current GPIO level (false = low/pressed, true = high/released)
    ///
    /// # Returns
    /// * `ButtonEvent` - Event indicating state change or none
    pub fn update(&mut self, gpio_level: bool) -> ButtonEvent {
        let new_raw_state = gpio_to_button_state(gpio_level);
        self.update_debounce_counter(new_raw_state);
        self.check_state_transition()
    }

    /// Updates debounce counter based on raw state stability.
    ///
    /// # Arguments
    /// * `new_raw_state` - New raw button state from GPIO
    fn update_debounce_counter(&mut self, new_raw_state: ButtonState) {
        if new_raw_state == self.raw_state {
            if self.debounce_count < self.debounce_threshold {
                self.debounce_count += 1;
            }
        } else {
            self.raw_state = new_raw_state;
            self.debounce_count = 0;
        }
    }

    /// Checks for state transition and returns event.
    ///
    /// # Returns
    /// * `ButtonEvent` - Event if transition occurred
    fn check_state_transition(&mut self) -> ButtonEvent {
        if self.debounce_count >= self.debounce_threshold && self.state != self.raw_state {
            self.apply_state_transition()
        } else {
            ButtonEvent::None
        }
    }

    /// Applies state transition and updates LED state.
    ///
    /// # Returns
    /// * `ButtonEvent` - Event indicating transition type
    fn apply_state_transition(&mut self) -> ButtonEvent {
        let old_state = self.state;
        self.state = self.raw_state;
        self.handle_transition_event(old_state, self.state)
    }

    /// Handles transition event and updates LED/press count.
    ///
    /// # Arguments
    /// * `old_state` - Previous button state
    /// * `new_state` - New button state
    ///
    /// # Returns
    /// * `ButtonEvent` - Event indicating transition type
    fn handle_transition_event(&mut self, old_state: ButtonState, new_state: ButtonState) -> ButtonEvent {
        match (old_state, new_state) {
            (ButtonState::Released, ButtonState::Pressed) => self.handle_press_event(),
            (ButtonState::Pressed, ButtonState::Released) => self.handle_release_event(),
            _ => ButtonEvent::None,
        }
    }

    /// Handles button press event.
    ///
    /// # Returns
    /// * `ButtonEvent` - Pressed event
    fn handle_press_event(&mut self) -> ButtonEvent {
        self.press_count += 1;
        self.led_state = LedState::On;
        ButtonEvent::Pressed
    }

    /// Handles button release event.
    ///
    /// # Returns
    /// * `ButtonEvent` - Released event
    fn handle_release_event(&mut self) -> ButtonEvent {
        self.led_state = LedState::Off;
        ButtonEvent::Released
    }

    /// Returns current debounced button state.
    ///
    /// # Returns
    /// * `ButtonState` - Current button state
    #[allow(dead_code)]
    pub fn state(&self) -> ButtonState {
        self.state
    }

    /// Returns current raw (unfiltered) button state.
    ///
    /// # Returns
    /// * `ButtonState` - Current raw button state
    #[allow(dead_code)]
    pub fn raw_state(&self) -> ButtonState {
        self.raw_state
    }

    /// Returns current LED state.
    ///
    /// # Details
    /// LED state is controlled by button press/release.
    /// LED turns on when button is pressed, off when released.
    ///
    /// # Returns
    /// * `LedState` - Current LED state
    pub fn led_state(&self) -> LedState {
        self.led_state
    }

    /// Returns current debounce delay.
    ///
    /// # Returns
    /// * `u64` - Debounce delay in milliseconds
    pub fn debounce_delay_ms(&self) -> u64 {
        self.debounce_delay_ms
    }

    /// Returns debounce threshold.
    ///
    /// # Returns
    /// * `u32` - Number of stable samples required
    #[allow(dead_code)]
    pub fn debounce_threshold(&self) -> u32 {
        self.debounce_threshold
    }

    /// Returns total button press count.
    ///
    /// # Returns
    /// * `u64` - Number of presses
    #[allow(dead_code)]
    pub fn press_count(&self) -> u64 {
        self.press_count
    }

    /// Checks if button is currently pressed.
    ///
    /// # Returns
    /// * `bool` - true if button is pressed
    #[allow(dead_code)]
    pub fn is_pressed(&self) -> bool {
        self.state == ButtonState::Pressed
    }

    /// Checks if button is currently released.
    ///
    /// # Returns
    /// * `bool` - true if button is released
    #[allow(dead_code)]
    pub fn is_released(&self) -> bool {
        self.state == ButtonState::Released
    }

    /// Checks if LED is currently on.
    ///
    /// # Returns
    /// * `bool` - true if LED is on
    #[allow(dead_code)]
    pub fn is_led_on(&self) -> bool {
        self.led_state == LedState::On
    }

    /// Checks if LED is currently off.
    ///
    /// # Returns
    /// * `bool` - true if LED is off
    #[allow(dead_code)]
    pub fn is_led_off(&self) -> bool {
        self.led_state == LedState::Off
    }

    /// Sets new debounce delay, clamped to valid range.
    ///
    /// # Arguments
    /// * `delay_ms` - New debounce delay in milliseconds
    #[allow(dead_code)]
    pub fn set_debounce_delay(&mut self, delay_ms: u64) {
        self.debounce_delay_ms = clamp_debounce_delay(delay_ms);
    }

    /// Sets new debounce threshold.
    ///
    /// # Arguments
    /// * `threshold` - New debounce threshold (samples required)
    #[allow(dead_code)]
    pub fn set_debounce_threshold(&mut self, threshold: u32) {
        self.debounce_threshold = threshold;
    }

    /// Resets button controller to initial state.
    ///
    /// # Details
    /// Clears all state and counters.
    /// Useful for re-initialization.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.state = ButtonState::Released;
        self.raw_state = ButtonState::Released;
        self.debounce_count = 0;
        self.press_count = 0;
        self.led_state = LedState::Off;
    }
}

/// Clamps debounce delay value to valid range.
///
/// # Details
/// Ensures delay falls within MIN_DEBOUNCE_DELAY_MS and MAX_DEBOUNCE_DELAY_MS.
///
/// # Arguments
/// * `delay_ms` - Delay to clamp
///
/// # Returns
/// * `u64` - Clamped delay value
fn clamp_debounce_delay(delay_ms: u64) -> u64 {
    delay_ms.clamp(MIN_DEBOUNCE_DELAY_MS, MAX_DEBOUNCE_DELAY_MS)
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

/// Converts boolean GPIO level to ButtonState.
///
/// # Details
/// Maps GPIO level to button state.
/// Active-low: low (false) = Pressed, high (true) = Released.
///
/// # Arguments
/// * `gpio_level` - GPIO level (false = low, true = high)
///
/// # Returns
/// * `ButtonState` - Corresponding button state
#[allow(dead_code)]
pub fn gpio_to_button_state(gpio_level: bool) -> ButtonState {
    if gpio_level {
        ButtonState::Released
    } else {
        ButtonState::Pressed
    }
}

/// Runs the main button polling loop.
///
/// # Arguments
/// * `button` - Button input GPIO reference
/// * `led` - LED output GPIO mutable reference
/// * `controller` - Button controller mutable reference
#[cfg(feature = "embassy-rp")]
pub async fn run_button_loop(
    button: &Input<'_>,
    led: &mut Output<'_>,
    controller: &mut ButtonController,
) {
    loop {
        process_button_state(button, led, controller);
        Timer::after_millis(controller.debounce_delay_ms()).await;
    }
}

/// Processes button state and updates LED.
///
/// # Arguments
/// * `button` - Button input GPIO reference
/// * `led` - LED output GPIO mutable reference
/// * `controller` - Button controller mutable reference
#[cfg(feature = "embassy-rp")]
fn process_button_state(
    button: &Input<'_>,
    led: &mut Output<'_>,
    controller: &mut ButtonController,
) {
    controller.update(button.is_high());
    update_led(led, controller);
}

/// Updates LED based on controller state.
///
/// # Arguments
/// * `led` - LED output GPIO mutable reference
/// * `controller` - Button controller reference
#[cfg(feature = "embassy-rp")]
fn update_led(led: &mut Output<'_>, controller: &ButtonController) {
    if led_state_to_level(controller.led_state()) {
        led.set_high();
    } else {
        led.set_low();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ButtonController Construction Tests ====================

    #[test]
    fn test_new_controller() {
        let ctrl = ButtonController::new();
        assert_eq!(ctrl.state(), ButtonState::Released);
        assert_eq!(ctrl.debounce_delay_ms(), DEBOUNCE_DELAY_MS);
        assert_eq!(ctrl.debounce_threshold(), DEBOUNCE_COUNT);
        assert_eq!(ctrl.press_count(), 0);
        assert_eq!(ctrl.led_state(), LedState::Off);
    }

    #[test]
    fn test_with_debounce() {
        let ctrl = ButtonController::with_debounce(20, 10);
        assert_eq!(ctrl.debounce_delay_ms(), 20);
        assert_eq!(ctrl.debounce_threshold(), 10);
    }

    #[test]
    fn test_with_debounce_clamps_low() {
        let ctrl = ButtonController::with_debounce(0, 5);
        assert_eq!(ctrl.debounce_delay_ms(), MIN_DEBOUNCE_DELAY_MS);
    }

    #[test]
    fn test_with_debounce_clamps_high() {
        let ctrl = ButtonController::with_debounce(1000, 5);
        assert_eq!(ctrl.debounce_delay_ms(), MAX_DEBOUNCE_DELAY_MS);
    }

    // ==================== Button State Tests ====================

    #[test]
    fn test_is_released_initially() {
        let ctrl = ButtonController::new();
        assert!(ctrl.is_released());
        assert!(!ctrl.is_pressed());
    }

    #[test]
    fn test_state_returns_released_initially() {
        let ctrl = ButtonController::new();
        assert_eq!(ctrl.state(), ButtonState::Released);
    }

    #[test]
    fn test_raw_state_returns_released_initially() {
        let ctrl = ButtonController::new();
        assert_eq!(ctrl.raw_state(), ButtonState::Released);
    }

    // ==================== LED State Tests ====================

    #[test]
    fn test_led_off_initially() {
        let ctrl = ButtonController::new();
        assert!(ctrl.is_led_off());
        assert!(!ctrl.is_led_on());
    }

    #[test]
    fn test_led_state_returns_off_initially() {
        let ctrl = ButtonController::new();
        assert_eq!(ctrl.led_state(), LedState::Off);
    }

    // ==================== Debounce Logic Tests ====================

    #[test]
    fn test_update_no_change_when_released() {
        let mut ctrl = ButtonController::new();
        let event = ctrl.update(true);
        assert_eq!(event, ButtonEvent::None);
        assert_eq!(ctrl.state(), ButtonState::Released);
    }

    #[test]
    fn test_update_requires_debounce_threshold() {
        let mut ctrl = ButtonController::with_debounce(5, 3);
        ctrl.update(false);
        assert_eq!(ctrl.state(), ButtonState::Released);
        ctrl.update(false);
        assert_eq!(ctrl.state(), ButtonState::Released);
        ctrl.update(false);
        assert_eq!(ctrl.state(), ButtonState::Released);
        let event = ctrl.update(false);
        assert_eq!(event, ButtonEvent::Pressed);
        assert_eq!(ctrl.state(), ButtonState::Pressed);
    }

    #[test]
    fn test_update_resets_debounce_on_change() {
        let mut ctrl = ButtonController::with_debounce(5, 3);
        ctrl.update(false);
        ctrl.update(false);
        ctrl.update(true);
        ctrl.update(false);
        assert_eq!(ctrl.state(), ButtonState::Released);
    }

    #[test]
    fn test_button_press_event() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        ctrl.update(false);
        let event = ctrl.update(false);
        assert_eq!(event, ButtonEvent::Pressed);
    }

    #[test]
    fn test_button_release_event() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        ctrl.update(false);
        ctrl.update(false);
        ctrl.update(true);
        let event = ctrl.update(true);
        assert_eq!(event, ButtonEvent::Released);
    }

    // ==================== Button-LED Interaction Tests ====================

    #[test]
    fn test_led_on_when_button_pressed() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        ctrl.update(false);
        ctrl.update(false);
        assert!(ctrl.is_led_on());
        assert_eq!(ctrl.led_state(), LedState::On);
    }

    #[test]
    fn test_led_off_when_button_released() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        ctrl.update(false);
        ctrl.update(false);
        ctrl.update(true);
        ctrl.update(true);
        assert!(ctrl.is_led_off());
        assert_eq!(ctrl.led_state(), LedState::Off);
    }

    #[test]
    fn test_led_follows_button_state() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        ctrl.update(false);
        ctrl.update(false);
        assert!(ctrl.is_pressed());
        assert!(ctrl.is_led_on());
        ctrl.update(true);
        ctrl.update(true);
        assert!(ctrl.is_released());
        assert!(ctrl.is_led_off());
        ctrl.update(false);
        ctrl.update(false);
        assert!(ctrl.is_pressed());
        assert!(ctrl.is_led_on());
    }

    // ==================== Press Count Tests ====================

    #[test]
    fn test_press_count_increments() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        ctrl.update(false); ctrl.update(false);
        ctrl.update(true);  ctrl.update(true);
        ctrl.update(false); ctrl.update(false);
        ctrl.update(true);  ctrl.update(true);
        ctrl.update(false); ctrl.update(false);
        assert_eq!(ctrl.press_count(), 3);
    }

    #[test]
    fn test_press_count_zero_initially() {
        let ctrl = ButtonController::new();
        assert_eq!(ctrl.press_count(), 0);
    }

    // ==================== Configuration Tests ====================

    #[test]
    fn test_set_debounce_delay() {
        let mut ctrl = ButtonController::new();
        ctrl.set_debounce_delay(15);
        assert_eq!(ctrl.debounce_delay_ms(), 15);
    }

    #[test]
    fn test_set_debounce_delay_clamps() {
        let mut ctrl = ButtonController::new();
        ctrl.set_debounce_delay(0);
        assert_eq!(ctrl.debounce_delay_ms(), MIN_DEBOUNCE_DELAY_MS);
    }

    #[test]
    fn test_set_debounce_threshold() {
        let mut ctrl = ButtonController::new();
        ctrl.set_debounce_threshold(10);
        assert_eq!(ctrl.debounce_threshold(), 10);
    }

    // ==================== Reset Tests ====================

    #[test]
    fn test_reset_clears_state() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        ctrl.update(false);
        ctrl.reset();
        assert_eq!(ctrl.state(), ButtonState::Released);
        assert_eq!(ctrl.raw_state(), ButtonState::Released);
        assert_eq!(ctrl.press_count(), 0);
        assert_eq!(ctrl.led_state(), LedState::Off);
    }

    // ==================== Utility Function Tests ====================

    #[test]
    fn test_led_state_to_level_on() {
        assert!(led_state_to_level(LedState::On));
    }

    #[test]
    fn test_led_state_to_level_off() {
        assert!(!led_state_to_level(LedState::Off));
    }

    #[test]
    fn test_gpio_to_button_state_high() {
        assert_eq!(gpio_to_button_state(true), ButtonState::Released);
    }

    #[test]
    fn test_gpio_to_button_state_low() {
        assert_eq!(gpio_to_button_state(false), ButtonState::Pressed);
    }

    #[test]
    fn test_clamp_debounce_delay_within_range() {
        assert_eq!(clamp_debounce_delay(10), 10);
    }

    #[test]
    fn test_clamp_debounce_delay_below_min() {
        assert_eq!(clamp_debounce_delay(0), MIN_DEBOUNCE_DELAY_MS);
    }

    #[test]
    fn test_clamp_debounce_delay_above_max() {
        assert_eq!(clamp_debounce_delay(1000), MAX_DEBOUNCE_DELAY_MS);
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_multiple_same_state_updates() {
        let mut ctrl = ButtonController::with_debounce(5, 1);
        for _ in 0..10 {
            let event = ctrl.update(true);
            assert_eq!(event, ButtonEvent::None);
        }
        assert!(ctrl.is_released());
    }

    #[test]
    fn test_rapid_press_release_with_debounce() {
        let mut ctrl = ButtonController::with_debounce(5, 5);
        ctrl.update(false);
        ctrl.update(true);
        ctrl.update(false);
        ctrl.update(true);
        ctrl.update(false);
        assert!(ctrl.is_released());
    }

    #[test]
    fn test_stable_press_with_debounce() {
        let mut ctrl = ButtonController::with_debounce(5, 5);
        for i in 0..6 {
            let event = ctrl.update(false);
            if i < 5 {
                assert_eq!(event, ButtonEvent::None);
            } else {
                assert_eq!(event, ButtonEvent::Pressed);
            }
        }
        assert!(ctrl.is_pressed());
    }

    #[test]
    fn test_debounce_count_saturates() {
        let mut ctrl = ButtonController::with_debounce(5, 3);
        for _ in 0..100 {
            ctrl.update(true);
        }
        assert!(ctrl.is_released());
    }

    // ==================== ButtonState Enum Tests ====================

    #[test]
    fn test_button_state_equality() {
        assert_eq!(ButtonState::Pressed, ButtonState::Pressed);
        assert_eq!(ButtonState::Released, ButtonState::Released);
        assert_ne!(ButtonState::Pressed, ButtonState::Released);
    }

    #[test]
    fn test_button_state_copy() {
        let state = ButtonState::Pressed;
        let copy = state;
        assert_eq!(state, copy);
    }

    // ==================== ButtonEvent Enum Tests ====================

    #[test]
    fn test_button_event_equality() {
        assert_eq!(ButtonEvent::Pressed, ButtonEvent::Pressed);
        assert_eq!(ButtonEvent::Released, ButtonEvent::Released);
        assert_eq!(ButtonEvent::None, ButtonEvent::None);
        assert_ne!(ButtonEvent::Pressed, ButtonEvent::Released);
    }

    #[test]
    fn test_button_event_copy() {
        let event = ButtonEvent::Pressed;
        let copy = event;
        assert_eq!(event, copy);
    }

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

    // ==================== Default Trait Tests ====================

    #[test]
    fn test_default_matches_new() {
        let default_ctrl = ButtonController::default();
        let new_ctrl = ButtonController::new();
        assert_eq!(default_ctrl.state(), new_ctrl.state());
        assert_eq!(default_ctrl.debounce_delay_ms(), new_ctrl.debounce_delay_ms());
        assert_eq!(default_ctrl.debounce_threshold(), new_ctrl.debounce_threshold());
        assert_eq!(default_ctrl.press_count(), new_ctrl.press_count());
        assert_eq!(default_ctrl.led_state(), new_ctrl.led_state());
    }
}
