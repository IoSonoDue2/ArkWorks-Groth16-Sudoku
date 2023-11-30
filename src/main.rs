use ark_bls12_377::{Bls12_377, Fr};
use std::time::Instant;
use ark_serialize::CanonicalSerialize;
use rand::thread_rng;
use ark_groth16::Groth16;
use ark_relations::r1cs::{ConstraintSynthesizer,ConstraintSystem};
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};

mod alloc;
mod cmp;
mod constraint;


#[derive(Clone)]
struct Circuit<const N: usize> {
    solution : [[u8;N];N],
    puzzle: [[u8;N];N],
}




fn serialize_input <const N: usize> (mat: [[u8;N];N]) -> Vec<Fr>{
    let mut enc_input = Vec::new();
    for row in mat.iter() {
        for &value in row.iter() {
            for i in 0..8 {
                if (value >> i) & 1 == 1 {
                    enc_input.push(Fr::from(1u8));
                } else {
                    enc_input.push(Fr::from(0u8));
                }
            }
        }
    }
    enc_input
    
}


fn main() {

    let puzzle = [();9].map(|_| [(); 9].map(|_| 0u8));
    let solution = [();9].map(|_| [(); 9].map(|_| 0u8));

    let circuit_defining_cs =Circuit{
        puzzle,
        solution,
    };
    println!("------------------------------------------------------------------------");
    println!("This test cheks if someone has a valid solution for a 9x9 sudoku puzzle");
    println!("------------------------------------------------------------------------");

    let rng = &mut thread_rng();

    println!("\n===Check circuit defining constraint system without proving===");
    let cs = ConstraintSystem::<Fr>::new_ref();
        // The function consumes the circuit and constraint system, which is why we clone them.
    circuit_defining_cs.clone().generate_constraints(cs.clone()).unwrap();
    println!("Is satisfied: {}", cs.is_satisfied().unwrap());
    println!("Num constraints: {}", cs.num_constraints());

    println!("\n===Entering Groth16 Setup Phase===");
    
    let start = Instant::now();
    let (pk, vk) = Groth16::<Bls12_377>::setup(circuit_defining_cs.clone(),rng).unwrap();
    let end = start.elapsed();

    println!("Setup time: {:?}", end);

    let puzzle = [
         [5, 3, 0, 0, 7, 0, 0, 0, 0],
         [6, 0, 0, 1, 9, 5, 0, 0, 0],
         [0, 9, 8, 0, 0, 0, 0, 6, 0],
         [8, 0, 0, 0, 6, 0, 0, 0, 3],
         [4, 0, 0, 8, 0, 3, 0, 0, 1],
         [7, 0, 0, 0, 2, 0, 0, 0, 6],
         [0, 6, 0, 0, 0, 0, 2, 8, 0],
         [0, 0, 0, 4, 1, 9, 0, 0, 5],
         [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];


    let solution = [
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ];


    let circuit_to_verify =Circuit{
        puzzle,
        solution,
    };

    println!("\n===Entering Proving Phase===");
    
    let mut start = Instant::now();
    let proof = Groth16::<Bls12_377>::prove(&pk, circuit_to_verify, rng).unwrap();
    let end = start.elapsed();

    println!("Proving time: {:?}", end);

    println!("Proof size: {}", proof.compressed_size());

    println!("\n===Verification Phase===");
    let enc_input =  serialize_input(puzzle.clone());
    println!("Public Input Lenght {}", enc_input.len());
    start = Instant::now();
    let is_valid=Groth16::<Bls12_377>::verify(&vk, &enc_input,&proof).unwrap();
    let end = start.elapsed();

    println!("Verification time: {:?}", end);

    assert!(is_valid);

    println!("The solution is valid\n");
    
    println!("------------------------------------------------------------------------");
    println!("This test cheks if someone has a valid solution for a 2x2 sudoku puzzle");
    println!("------------------------------------------------------------------------");


    
    let puzzle = [();2].map(|_| [(); 2].map(|_| 0u8));
    let solution = [();2].map(|_| [(); 2].map(|_| 0u8));

    let circuit_defining_cs =Circuit{
        puzzle,
        solution,
    };



    let rng = &mut thread_rng();

    println!("\n===Check circuit defining constraint system without proving===");
    let cs = ConstraintSystem::<Fr>::new_ref();
        // The function consumes the circuit and constraint system, which is why we clone them.
    circuit_defining_cs.clone().generate_constraints(cs.clone()).unwrap();
    println!("Is satisfied: {}", cs.is_satisfied().unwrap());
    println!("Num constraints: {}", cs.num_constraints());

    println!("\n===Entering Groth16 Setup Phase===");
    start = Instant::now();
    let (pk, vk) = Groth16::<Bls12_377>::setup(circuit_defining_cs.clone(),rng).unwrap();
    let end = start.elapsed();
    println!("Setup time: {:?}", end);
    let puzzle = [
        [0,1],
        [0,2],
    ];


    let solution = [
        [2,1],
        [1,2],
    ];


    let circuit_to_verify =Circuit{
        puzzle,
        solution,
    };

    println!("\n===Entering Proving Phase===");
    start = Instant::now();
    let proof = Groth16::<Bls12_377>::prove(&pk, circuit_to_verify, rng).unwrap();
    let end = start.elapsed();
    println!("Proving time: {:?}", end);
    println!("Proof size: {}", proof.compressed_size());



    println!("\n===Verification Phase===");
    let enc_input =  serialize_input(puzzle.clone());
    println!("Public Input Lenght {}", enc_input.len());
    start = Instant::now();
    let is_valid = Groth16::<Bls12_377>::verify(&vk, &enc_input,&proof).unwrap();
    let end = start.elapsed();

    println!("Verification time: {:?}", end);
    assert!(is_valid);
    println!("The solution is valid");

        
    println!("------------------------------------------------------------------------");
    println!("This test cheks if someone has a valid solution for a 16x16 sudoku puzzle");
    println!("------------------------------------------------------------------------");


    
    let puzzle = [();16].map(|_| [(); 16].map(|_| 0u8));
    let solution = [();16].map(|_| [(); 16].map(|_| 0u8));

    let circuit_defining_cs =Circuit{
        puzzle,
        solution,
    };



    let rng = &mut thread_rng();

    println!("\n===Check circuit defining constraint system without proving===");
    let cs = ConstraintSystem::<Fr>::new_ref();
        // The function consumes the circuit and constraint system, which is why we clone them.
    circuit_defining_cs.clone().generate_constraints(cs.clone()).unwrap();
    println!("Is satisfied: {}", cs.is_satisfied().unwrap());
    println!("Num constraints: {}", cs.num_constraints());

    println!("\n===Entering Groth16 Setup Phase===");
    start = Instant::now();
    let (pk, vk) = Groth16::<Bls12_377>::setup(circuit_defining_cs.clone(),rng).unwrap();
    let end = start.elapsed();
    println!("Setup time: {:?}", end);
    let puzzle = [
     [ 3,  1, 14, 0,  4, 13,  2, 16,  9, 11,  6,  7,  5,  8, 12, 15,],
     [0 ,11,  7,  4, 10,  3, 14,  1, 16,  5,  8, 12,  2,  9, 13,  6,],
     [12  ,6, 16, 13,  5,  7,  9,  8,  3, 0,  2, 15, 0,  1,  4, 14,],
     [ 8 , 5,  2,  9, 11, 12,  6, 15,  4,  1, 14, 13,  3, 16, 10,  7,],
     [ 1, 15, 11,  6,  7, 16,  3, 14, 10, 13, 12,  5,  4,  2,  8,  9,],
     [14,  3,  5, 12, 15, 10, 13,  4,  1,  8,  9,  2,  7, 11,  6, 16,],
     [ 2,  4,  8,  7, 12, 11,  1,  9, 14, 15, 16,  0, 13,  3,  5, 10,],
     [13, 10,  9, 0,  8,  2,  5,  6, 11,  3,  7,  4, 15, 14,  1, 12,],
     [ 0, 13, 15, 0,  9,  8, 12, 10,  2, 16,  3,  1,  6,  7, 14,  4,],
     [ 9,  8, 12,  3, 13,  1, 11,  7,  5,  6,  4, 14, 10, 15, 0,  2,],
     [10, 14,  6,  1,  2,  4, 16,  5,  7,  9, 15,  0, 12, 13, 11,  3,],
     [16,  7,  4,  2,  6, 14, 0,  3, 13, 12, 11, 10,  0,  5,  9,  1,],
     [ 6, 16,  3,  8, 14,  5,  4, 12, 15,  2,  1, 11,  9, 10,  7, 13,],
     [11, 12,  0, 14,  3, 15, 10, 13,  6,  7,  5,  0, 16,  4,  0,  8,],
     [ 7,  2, 13, 0,  1,  9,  8, 11, 12,  4, 10, 16, 14,  6,  0,  5,],
     [ 4,  9, 10,  5, 16,  6,  7,  2,  0, 14, 13,  3,  1, 12, 15, 11,],];



     let solution = [
     [ 3,  1, 14, 10,  4, 13,  2, 16,  9, 11,  6,  7,  5,  8, 12, 15,],
     [15 ,11,  7,  4, 10,  3, 14,  1, 16,  5,  8, 12,  2,  9, 13,  6,],
     [12  ,6, 16, 13,  5,  7,  9,  8,  3, 10,  2, 15, 11,  1,  4, 14,],
     [ 8 , 5,  2,  9, 11, 12,  6, 15,  4,  1, 14, 13,  3, 16, 10,  7,],
     [ 1, 15, 11,  6,  7, 16,  3, 14, 10, 13, 12,  5,  4,  2,  8,  9,],
     [14,  3,  5, 12, 15, 10, 13,  4,  1,  8,  9,  2,  7, 11,  6, 16,],
     [ 2,  4,  8,  7, 12, 11,  1,  9, 14, 15, 16,  6, 13,  3,  5, 10,],
     [13, 10,  9, 16,  8,  2,  5,  6, 11,  3,  7,  4, 15, 14,  1, 12,],
     [ 5, 13, 15, 11,  9,  8, 12, 10,  2, 16,  3,  1,  6,  7, 14,  4,],
     [ 9,  8, 12,  3, 13,  1, 11,  7,  5,  6,  4, 14, 10, 15, 16,  2,],
     [10, 14,  6,  1,  2,  4, 16,  5,  7,  9, 15,  8, 12, 13, 11,  3,],
     [16,  7,  4,  2,  6, 14, 15,  3, 13, 12, 11, 10,  8,  5,  9,  1,],
     [ 6, 16,  3,  8, 14,  5,  4, 12, 15,  2,  1, 11,  9, 10,  7, 13,],
     [11, 12,  1, 14,  3, 15, 10, 13,  6,  7,  5,  9, 16,  4,  2,  8,],
     [ 7,  2, 13, 15,  1,  9,  8, 11, 12,  4, 10, 16, 14,  6,  3,  5,],
     [ 4,  9, 10,  5, 16,  6,  7,  2,  8, 14, 13,  3,  1, 12, 15, 11,],];



    let circuit_to_verify =Circuit{
        puzzle,
        solution,
    };

    println!("\n===Entering Proving Phase===");

    start = Instant::now();
    let proof = Groth16::<Bls12_377>::prove(&pk, circuit_to_verify, rng).unwrap();
    let end = start.elapsed();
    println!("Proving time: {:?}", end);

    println!("Proof size: {}", proof.compressed_size());

    println!("\n===Verification Phase===");
    let enc_input =  serialize_input(puzzle.clone());
    println!("Public Input Lenght {}", enc_input.len());
    start = Instant::now();
    let is_valid= Groth16::<Bls12_377>::verify(&vk, &enc_input,&proof).unwrap();
    let end = start.elapsed();
    println!("Verification time: {:?}", end);
    assert!(is_valid);
    println!("The solution is valid");



}
