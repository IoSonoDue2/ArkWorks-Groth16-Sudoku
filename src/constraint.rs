use ark_ff::PrimeField;
use ark_r1cs_std::{
    prelude::{Boolean, EqGadget, AllocVar},
    uint8::UInt8,
};
use ark_relations::r1cs::{ConstraintSynthesizer,ConstraintSystemRef, SynthesisError, Result as ArkResult,};
use crate::cmp::CmpGadget;
use crate::Circuit;




#[derive(Clone)]
pub struct Solution<const N: usize, ConstraintF:PrimeField>(pub [[UInt8<ConstraintF>; N]; N]); 

#[derive(Clone)]
pub struct Puzzle<const N: usize, ConstraintF: PrimeField>(pub [[UInt8<ConstraintF>; N]; N]);




fn check_rows <const N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, ConstraintF>,
) -> Result<(), SynthesisError>{

    for row in &solution.0{
        for (j, cell) in row.iter().enumerate(){
            for prior_cell in &row[0..j]{
                cell.is_neq(&prior_cell)?
                    .enforce_equal(&Boolean::TRUE)?;
            }
        }
    }
    Ok(())
}

fn check_col <const N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, ConstraintF>,
) -> Result<(), SynthesisError>{
    for col in 0..N{
        for (j, row) in solution.0.iter().enumerate(){
            for prior_row in solution.0.iter().take(j){
                row[col].is_neq(&prior_row[col])?
                    .enforce_equal(&Boolean::TRUE)?;
            }
        }
    }
    Ok(())
}


fn check_squares<const N: usize, ConstraintF: PrimeField>(
    solution: &Solution<N, ConstraintF>,
) -> Result<(), SynthesisError>{

    let x: f32 = N as f32;

    let n = x.sqrt() as usize; 
    
    for i in 0..n {
        for j in 0..n {

            for row_idx in (i*n)..((i+1)*n){
                for col_idx in (j*n)..((j+1)*n){
                    for prior_row_idx in (i*n)..(row_idx){
                        for prior_col_idx in (j*n)..(col_idx){
                            solution.0[row_idx][col_idx].is_neq(&solution.0[prior_row_idx][prior_col_idx])?
                                .enforce_equal(&Boolean::TRUE)?;
                            
                        }
                    }

                }
            }
        
        }
    }
    Ok(())

}

fn check_puzzle_matches_solution<const N: usize, ConstraintF: PrimeField>(
    puzzle: &Puzzle<N, ConstraintF>,
    solution: &Solution<N, ConstraintF>,
) -> Result<(), SynthesisError>{

    for (p_row, s_row) in puzzle.0.iter().zip(&solution.0){
        for (p, s) in p_row.iter().zip(s_row){
         
            s.is_leq(&UInt8::constant(N as u8))?
                .and(&s.is_geq(&UInt8::constant(1))?)?
                .enforce_equal(&Boolean::TRUE)?;
            (p.is_eq(s)?.or(&p.is_eq(&UInt8::constant(0))?)?)
                .enforce_equal(&Boolean::TRUE)?;
        }
    }
    Ok(())
}

impl<const N: usize, F:PrimeField> ConstraintSynthesizer<F> for Circuit<N> 
{
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> ArkResult<()> {

        let puzzle_var = Puzzle::new_input(cs.clone(), || Ok(self.puzzle)).unwrap();
        let solution_var = Solution::new_witness(cs.clone(), || Ok(self.solution)).unwrap();

        check_puzzle_matches_solution::<N,F>(&puzzle_var,&solution_var).unwrap();
        check_rows::<N,F>(&solution_var).unwrap();
        check_col::<N,F>(&solution_var).unwrap();
        check_squares::<N,F>(&solution_var).unwrap();
        Ok(())


    }
} 


