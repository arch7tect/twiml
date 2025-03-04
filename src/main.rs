use crate::twiml::ToXmlString;

mod twiml;

fn main() {
    // Example 1: Simple voice response
    let response1 = twiml::Response::new()
        .say("Welcome to our service")
        .redirect("/next-step");
    
    println!("Simple Response:\n{}", response1.to_xml_string());

    // Example 2: Complex IVR
    let response2 = twiml::Response::new()
        .say("Please enter your account number")
        .gather(
            twiml::Gather::new()
                .action("/process-input")
                .method("POST")
                .num_digits("4")
                .say("Enter your 4-digit account number")
        )
        .redirect("/fallback");
    
    println!("\nComplex Response:\n{}", response2.to_xml_string());
}
