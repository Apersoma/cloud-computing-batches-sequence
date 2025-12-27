// use crate::batches::{self, Batches};




// pub enum LazyBatches {
//     Batches(Batches),
//     ShrinkBatches {
//         base: Box<LazyBatches>,
//         via: BatchShrinkMethod,
//     },
//     PhiOmicron {
//         base: Box<LazyBatches>
//     },
//     PhiIsTwo {
//         omicron: usize
//     },
//     PhiSquared {
//         phi: usize
//     },
//     PhiSquaredSubPhiAdd1 {
//         phi: usize
//     }   
// }

// impl LazyBatches {
//     pub fn phi(&self) -> usize {
//         match self {
//             Self::Batches(batches) => batches.phi,
//             Self::PhiIsTwo {..} => 2,
//             Self::PhiOmicron { base } => base.phi(),
//             Self::PhiSquared { phi } => *phi,
//             Self::PhiSquaredSubPhiAdd1 { phi } => *phi,
//             Self::ShrinkBatches { base, via } => match via {
//                 BatchShrinkMethod::PhiSquared => {
//                     let base_phi = base.phi();
//                     let phi = base_phi.isqrt();
//                     assert_eq!(phi*phi, base_phi);
//                     return phi;
//                 },
//                 BatchShrinkMethod::PhiOmicron { phi } => *phi,
//                 BatchShrinkMethod::PhiSquaredSubPhiAdd1 =>  {

//                 },
//                 BatchShrinkMethod::PhiIsTwo => 2,
//             },
//         }
//     }
//     pub fn phi_unchecked(&self) -> usize {

//     }
// }

// pub enum BatchShrinkMethod {
//     PhiSquared,
//     PhiOmicron {phi: usize},
//     PhiSquaredSubPhiAdd1,
//     PhiIsTwo,
// }

// pub enum Generation {
//     Phi,
//     PhiSquared,
//     PhiOmicron,
//     PhiSquaredSubPhiAdd1,
//     PhiIsTwo,
// }