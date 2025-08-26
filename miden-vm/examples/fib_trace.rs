//! Example: Run 10 rounds of Fibonacci in Miden VM and print the execution trace.

use miden_processor::ExecutionOptions;
use miden_vm::{execute, AdviceInputs, Assembler, DefaultHost, StackInputs};

fn main() {
    // Define the Miden Assembly program for calculating Fibonacci sequence
    // This program computes 10 iterations of Fibonacci numbers
    // Algorithm: starts with 0,1 and repeatedly computes next = prev + curr
    let masm_code = r#"
        begin
            push.0      # Initialize with fib(0) = 0
            push.1      # Initialize with fib(1) = 1
            
            repeat.10   # Repeat 10 times to get fib(11)
                dup.1   # Duplicate the second element (previous number)
                add     # Add: curr + prev = next
                swap    # Move next to correct position
                drop    # Remove the old previous number
            end
        end
    "#;

    // Assemble the MASM code into a program
    let program = Assembler::default()
        .assemble_program(masm_code)
        .expect("Failed to compile Miden Assembly code");

    // Set up execution environment
    let stack_inputs = StackInputs::default(); // Empty initial stack
    let advice_inputs = AdviceInputs::default(); // No advice inputs needed
    let mut host = DefaultHost::default(); // Default execution host
    let options = ExecutionOptions::default(); // Default execution options

    // Execute the program and capture the trace
    let trace = execute(
        &program,
        stack_inputs,
        advice_inputs,
        &mut host,
        options,
    )
    .expect("Program execution failed");

    // Output the execution trace
    println!("=== Miden VM Fibonacci Execution Trace ===");
    // println!("{:?}", trace);
    trace.print();
}
