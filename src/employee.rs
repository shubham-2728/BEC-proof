use crate::proof::Proof;

// employee.rs
pub struct Employee {
    pub employee_id: String,
    pub name: String,
    pub email: String,
    pub business_suffix: String,
    pub proof: Proof,
}