// Import the compiled WASM module. wasm-bindgen generated this JS wrapper.
// After import, `wasm.check_prime` is a JS function that calls into Rust.
import * as wasm from "wasm-is-prime";

// ---------------------------------------------------------------------------
// DOM References
// Cache these once at startup rather than querying the DOM on every click.
// ---------------------------------------------------------------------------
const textbox1 = document.getElementById("PrimeNumber");
const canvas   = document.getElementById("board");
const ctx      = canvas.getContext("2d"); // 2D drawing context

// ---------------------------------------------------------------------------
// drawAnswer(yn: number): void
//
// Renders a colored circle with a result symbol onto the canvas.
//
// Parameters:
//   yn - 1 if the input was prime (green + checkmark)
//        0 if the input was not prime (red + X)
// ---------------------------------------------------------------------------
function drawAnswer(yn) {
    // Clear previous drawing before rendering new result
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Center coordinates and radius for the indicator circle
    const xpos   = 50;
    const ypos   = 50;
    const radius = 25;

    // Draw filled circle
    ctx.beginPath();
    ctx.arc(xpos, ypos, radius, 0, 2 * Math.PI);
    ctx.fillStyle = (yn === 1) ? 'green' : 'red';
    ctx.fill();

    // Draw result symbol centered inside the circle
    ctx.font        = '24pt Calibri';
    ctx.fillStyle   = 'white';
    ctx.textAlign   = 'center';
    ctx.textBaseline = 'middle'; // Vertically center the text

    // Checkmark for prime, X for not prime
    const symbol = (yn === 1) ? '✅' : '❌';
    ctx.fillText(symbol, xpos, ypos);
}

// ---------------------------------------------------------------------------
// Event Handler: "Check Number" button click
//
// Reads user input, delegates primality check to Rust via WASM,
// then renders the visual result on the canvas.
// ---------------------------------------------------------------------------
document.getElementById("CheckNumber").addEventListener("click", event => {
    // Pass the raw string value to Rust.
    // Rust's check_prime() handles parsing and validation internally.
    const answer = wasm.check_prime(textbox1.value);

    // Render result: green checkmark if prime, red X if not
    drawAnswer(answer);
});