// proof.rs
use sha2::{Sha256, Digest};

use crate::employee::Employee;

pub struct Proof {
    pub proof_data: Vec<u8>,
    pub public_inputs: Vec<u8>,
}

pub fn generate_proof(employee: &Employee) -> Proof {
    let mut hasher = Sha256::new();
    hasher.update(&employee.employee_id);
    hasher.update(&employee.name);
    hasher.update(&employee.email);
    hasher.update(&employee.business_suffix);
    let hashed_attributes = hasher.finalize();

    // Generate a zero-knowledge proof based on hashed attributes (pseudo-code)

    Proof {
        proof_data: hashed_attributes.to_vec(),
        public_inputs: hashed_attributes.to_vec(),
    }
}

pub fn verify_proof(proof: &Proof, employee_hash: &[u8]) -> bool {
    // Verify proof against employee_hash (pseudo-code)
    proof.public_inputs == employee_hash
}
