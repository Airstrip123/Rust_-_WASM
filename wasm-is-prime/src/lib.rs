// Pull in wasm_bindgen's prelude — this gives us JsValue, the #[wasm_bindgen]
// macro, and the extern "C" block capability.
use wasm_bindgen::prelude::*;

// ---------------------------------------------------------------------------
// JavaScript Interop: Import browser APIs into Rust's namespace
// The `extern "C"` block with #[wasm_bindgen] declares functions that
// exist in JavaScript land. Rust will call through to them at runtime.
// ---------------------------------------------------------------------------
#[wasm_bindgen]
extern "C" {
    /// Calls browser's window.alert(). The &str is automatically
    /// converted to a JS string by wasm-bindgen.
    fn alert(s: &str);
}

// ---------------------------------------------------------------------------
// Primality Test
//
// Returns 1 if n is prime, 0 otherwise.
// This is a private function (no `pub`, no #[wasm_bindgen]) — it is
// internal implementation detail not exposed to JavaScript.
//
// Algorithm: Trial division up to sqrt(n).
// - 0 and 1 are not prime by definition.
// - 2 is the only even prime.
// - All other even numbers are composite.
// - Only odd divisors need checking after handling 2.
// ---------------------------------------------------------------------------
fn is_prime(n: u32) -> u32 {
    // 0 and 1 are not prime by mathematical definition
    if n < 2 {
        return 0;
    }
    // 2 is the smallest and only even prime number
    if n == 2 {
        return 1;
    }
    // All other even numbers are divisible by 2, hence composite
    if n % 2 == 0 {
        return 0;
    }

    // Check odd divisors from 3 up to sqrt(n).
    // If no divisor found in this range, n is prime.
    // We use i * i <= n to avoid floating-point sqrt.
    let mut i = 3u32;
    while i * i <= n {
        if n % i == 0 {
            return 0; // Found a divisor: n is composite
        }
        i += 2; // Only odd divisors (3, 5, 7, 9, ...)
    }

    1 // No divisor found: n is prime
}

// ---------------------------------------------------------------------------
// Public API: check_prime
//
// This function is exported to JavaScript via #[wasm_bindgen].
// It accepts a JsValue (JavaScript's dynamic type) because the input
// comes from an HTML text field, which is always a JS string.
//
// Returns:
//   1  → input is a prime number
//   0  → input is not prime, or could not be parsed
// ---------------------------------------------------------------------------
#[wasm_bindgen]
pub fn check_prime(s: &JsValue) -> u32 {
    // JsValue::as_string() converts JS string to Rust Option<String>.
    // unwrap() is safe here because HTML text inputs always produce strings.
    let input: String = s.as_string().unwrap();

    // Attempt to parse the string as an unsigned 32-bit integer.
    // parse::<u32>() returns Result<u32, ParseIntError>.
    match input.parse::<u32>() {
        Ok(num) => {
            // Successfully parsed: perform primality test
            if is_prime(num) == 1 {
                return 1; // Prime: JS will draw green circle
            } else {
                return 0; // Not prime: JS will draw red circle
            }
        }
        Err(_) => {
            // Input was not a valid non-negative integer
            alert(&format!("Could not parse '{}' as a positive integer", input));
            return 0;
        }
    }
}