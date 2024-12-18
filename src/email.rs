// email.rs
use crate::proof::Proof;
use crate::employee::{self, Employee};

pub struct OutgoingEmail {
    pub is_proven: bool,
    pub content: String,
}


pub fn send_email(employee: &Employee, _proof: &Proof) -> Option<OutgoingEmail> {
    //Perform verification check.
    let employee_hash = crate::repository::get_employee_hash(&employee);
    let is_proven = crate::proof::verify_proof(_proof, &employee_hash);
    
    if is_proven {
        println!("Email for {} has been proven. Sending...", employee.name);
        Some(OutgoingEmail {
            is_proven,
            content: format!("This is a verified email from {}", employee.name),
        })
    } else {
        println!("Warning: Email for {} cannot be proven. Aborting send.", employee.name);
        None
    }
}

pub fn receive_email(email: OutgoingEmail) {
    if email.is_proven {
        println!("Received a verified email: {}", email.content);
    } else {
        println!("Warning: Received an email that could not be verified.");
    }
}

// pub struct IncomingEmail{
//     pub business_suffix: String,

//     pub proof: Proof,

//     pub employee_id: String,

//     pub name: String,

//     pub email: String,
// }
