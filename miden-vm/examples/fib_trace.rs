//! Example: Run 10 rounds of Fibonacci in Miden VM and print the execution trace.

use std::time::Instant;

use miden_processor::ExecutionOptions;
use miden_prover::{ProvingOptions, prove};
use miden_verifier::verify;
use miden_vm::{AdviceInputs, Assembler, DefaultHost, ProgramInfo, StackInputs, execute};

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
    let trace = execute(&program, stack_inputs.clone(), advice_inputs.clone(), &mut host, options)
        .expect("Program execution failed");

    // Output the execution trace
    println!("=== Miden VM Fibonacci Execution Trace ===");
    // println!("{:?}", trace);
    trace.print();

    // Generate proof
    println!("\n=== Generating Proof ===");
    let proving_options = ProvingOptions::default();
    let mut host_for_proving = DefaultHost::default();

    let proof_start = Instant::now();
    let (stack_outputs, proof) = prove(
        &program,
        stack_inputs.clone(),
        advice_inputs.clone(),
        &mut host_for_proving,
        proving_options,
    )
    .expect("Failed to generate proof");
    let proof_time = proof_start.elapsed();

    println!("Proof generated successfully!");
    println!("Proof generation time: {:?}", proof_time);
    println!("Stack outputs: {:?}", stack_outputs);

    // Verify the proof
    println!("\n=== Verifying Proof ===");
    let program_info: ProgramInfo = program.into();

    let verify_start = Instant::now();
    match verify(program_info, stack_inputs, stack_outputs.clone(), proof) {
        Ok(security_level) => {
            let verify_time = verify_start.elapsed();
            println!("✓ Proof verification successful!");
            println!("Verification time: {:?}", verify_time);
            println!("Security level: {} bits", security_level);
            println!("Final Fibonacci result: {}", stack_outputs[0]);
        },
        Err(e) => {
            let verify_time = verify_start.elapsed();
            println!("✗ Proof verification failed: {:?}", e);
            println!("Verification time: {:?}", verify_time);
        },
    }
}
