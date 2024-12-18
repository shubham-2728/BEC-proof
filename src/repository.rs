use sha2::{Digest, Sha256};

use crate::employee::Employee;

pub fn get_employee_hash(employee: &Employee) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&employee.employee_id);
    hasher.update(&employee.name);
    hasher.update(&employee.email);
    hasher.update(&employee.business_suffix);
    hasher.finalize().to_vec()
}
