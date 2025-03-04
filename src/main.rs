mod twiml;

use crate::twiml::{Say, Response, Gather, Dial, Conference, Number, Record, Play, Client, Pause, Redirect, ToXmlString, Message, Body};

fn main() {
    // Example 1: Simple voice response
    let response1 = Response::new()
        .say(Say::new("Welcome to our service")
            .voice("alice")
            .language("en-US"))
        .redirect(Redirect::new("/next-step"));
    
    println!("Example 1: Simple Response\n{}\n", response1.to_xml_string());

    // Example 2: Gather input with speech recognition
    let response2 = Response::new()
        .say(Say::new("Please tell us what you'd like to do today"))
        .gather(
            Gather::new()
                .action("/process-speech")
                .method("POST")
                .input("speech")
                .language("en-US")
                .hints("support, sales, billing")
                .say(Say::new("You can say support, sales, or billing"))
        )
        .redirect(Redirect::new("/fallback"));
    
    println!("Example 2: Speech Recognition\n{}\n", response2.to_xml_string());

    // Example 3: More complex IVR with DTMF
    let response3 = Response::new()
        .say(Say::new("Welcome to ACME Company").voice("alice"))
        .gather(
            Gather::new()
                .action("/menu-selection")
                .method("POST")
                .num_digits("1")
                .timeout(10)
                .say(
                    Say::new("For sales, press 1. For support, press 2. For billing, press 3.")
                        .voice("alice")
                        .loop_times(3)
                )
        )
        .redirect(Redirect::new("/timeout"));
    
    println!("Example 3: DTMF Menu\n{}\n", response3.to_xml_string());

    // Example 4: Conference call
    let response4 = Response::new()
        .say(Say::new("You are about to join the conference."))
        .dial(
            Dial::new(None::<String>)
                .conference(
                    Conference::new("Room123")
                        .muted(false)
                        .start_conference_on_enter(true)
                        .end_conference_on_exit(false)
                        .max_participants(10)
                        .beep(true)
                        .record("record-from-start")
                )
        );
    
    println!("Example 4: Conference Call\n{}\n", response4.to_xml_string());

    // Example 5: Recording a message
    let response5 = Response::new()
        .say(Say::new("Please leave a message after the tone. Press any key to finish."))
        .record(
            Record::new()
                .action("/handle-recording")
                .method("POST")
                .max_length(30)
                .finish_on_key("*#")
                .play_beep(true)
                .transcribe(true)
                .transcribe_callback("/transcription-callback")
        )
        .say(Say::new("Thank you for your message."));
    
    println!("Example 5: Recording\n{}\n", response5.to_xml_string());

    // Example 6: Connecting to phone numbers and clients
    let response6 = Response::new()
        .say(Say::new("Connecting you to sales."))
        .dial(
            Dial::new_empty()
                .timeout(20)
                .caller_id("+15551234567")
                .action("/handle-dial-status")
                .method("POST")
                .record("record-from-answer")
                .number(
                    Number::new("+18005551234")
                        .send_digits("1234#")
                        .url("/number-status")
                )
                .client(
                    Client::new("sales_department")
                        .status_callback("/client-status")
                )
        );
    
    println!("Example 6: Connecting to Multiple Destinations\n{}\n", response6.to_xml_string());

    // Example 7: SMS Message
    let response7 = Response::new()
        .message(
            Message::new_empty()
                .to("+15551234567")
                .from("+15559876543")
                .action("/message-status")
                .method("POST")
                .body(Body::new("Your appointment is confirmed for tomorrow at 2pm."))
        );
    
    println!("Example 7: SMS Message\n{}\n", response7.to_xml_string());

    // Example 8: Playing audio files with nested gather
    let response8 = Response::new()
        .play(Play::new("https://api.example.com/sounds/greeting.mp3"))
        .gather(
            Gather::new()
                .action("/process-selection")
                .method("POST")
                .timeout(10)
                .num_digits("1")
                .say(Say::new("Press a number to continue"))
                .play(Play::new("https://api.example.com/sounds/options.mp3"))
                .pause(Pause::new().length(1))
        );
    
    println!("Example 8: Playing Audio with Gather\n{}\n", response8.to_xml_string());
}
