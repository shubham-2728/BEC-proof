mod email;
mod proof;
mod repository;
mod employee;

use email::{send_email, receive_email, OutgoingEmail};
use proof::{generate_proof, verify_proof, Proof};
use repository::get_employee_hash;
use employee::Employee;

fn main() {
    // Example employee data
    let mut employee = Employee {
        employee_id: "12345".to_string(),
        name: "John Doe".to_string(),
        email: "johndoe@business.com".to_string(),
        business_suffix: "business.com".to_string(),
        proof: Proof {
            proof_data: Vec::new(),
            public_inputs: Vec::new(),
        }, // Initialize with an empty Proof
    };

    // Step 2: Generate the proof and update the employee
    employee.proof = generate_proof(&employee);


    // Verify incoming emails
    if let Some(outgoing_email) = send_email(&employee, &employee.proof) {
        // Simulate receiving the email with only verification status
        receive_email(outgoing_email);
    } else {
        println!("Email sending aborted due to failed verification.");
    }
}
