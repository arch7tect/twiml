use xml_builder::{XMLBuilder, XMLElement, XMLVersion};
use std::fmt::Debug;

/// Custom trait for XML string conversion
pub trait ToXmlString {
    /// Convert the element to an XML string
    fn to_xml_string(&self) -> String;
}

/// Trait for TwiML elements that can be converted to XML
pub trait TwiMLElement: Debug {
    /// Convert the element to an XMLElement
    fn to_xml(&self) -> XMLElement;
}

/// Implement ToXmlString for TwiMLElement
impl<T: TwiMLElement> ToXmlString for T {
    fn to_xml_string(&self) -> String {
        let mut xml = XMLBuilder::new()
            .version(XMLVersion::XML1_1)
            .encoding("UTF-8".into())
            .break_lines(false)
            .indent(false)
            .build();
        
        xml.set_root_element(self.to_xml());
        
        let mut writer = Vec::new();
        xml.generate(&mut writer).unwrap();
        
        String::from_utf8(writer).unwrap()
    }
}

/// Module containing all TwiML element implementations
mod elements {
    use super::*;

    #[derive(Debug)]
    struct ElementFactory {
        element: String,
        text: Option<String>,
        attributes: Vec<(String, String)>,
        children: Vec<Box<dyn TwiMLElement>>,
    }

    impl ElementFactory {
        /// Create a new ElementFactory element
        pub fn new(element: impl Into<String>, text: Option<impl Into<String>>) -> Self {
            Self {
                element: element.into(),
                text: if let Some(text) = text {Some(text.into())} else {None},
                attributes: Vec::new(),
                children: Vec::new(),
            }
        }
    }

    impl TwiMLElement for ElementFactory {
        fn to_xml(&self) -> XMLElement {
            let mut elem = XMLElement::new(&self.element);

            // Add attributes
            for (key, value) in &self.attributes {
                elem.add_attribute(key, value);
            }

            // Add children
            for child in &self.children {
                elem.add_child(child.to_xml()).unwrap();
            }

            // Add text (use owned String)
            if let Some(text) = self.text.clone() {
                elem.add_text(text).unwrap();
            }

            elem
        }
    }

    /// Say TwiML Element for text-to-speech
    #[derive(Debug)]
    pub struct Say {
        factory: ElementFactory,
    }

    impl Say {
        /// Create a new Say element
        pub fn new(text: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Say", Some(text)),
            }
        }

        /// Set voice attribute
        pub fn voice(mut self, voice: impl Into<String>) -> Self {
            self.factory.attributes.push(("voice".to_string(), voice.into()));
            self
        }

        /// Set language attribute
        pub fn language(mut self, language: impl Into<String>) -> Self {
            self.factory.attributes.push(("language".to_string(), language.into()));
            self
        }
        
        /// Set loop attribute
        pub fn loop_times(mut self, loops: usize) -> Self {
            self.factory.attributes.push(("loop".to_string(), loops.to_string()));
            self
        }
        
        /// Set pitch attribute
        pub fn pitch(mut self, pitch: impl Into<String>) -> Self {
            self.factory.attributes.push(("pitch".to_string(), pitch.into()));
            self
        }
        
        /// Set rate attribute
        pub fn rate(mut self, rate: impl Into<String>) -> Self {
            self.factory.attributes.push(("rate".to_string(), rate.into()));
            self
        }
    }

    impl TwiMLElement for Say {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Gather TwiML Element for collecting user input
    #[derive(Debug)]
    pub struct Gather {
        factory: ElementFactory,
    }

    impl Gather {
        /// Create a new Gather element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Gather", None::<String>),
            }
        }

        /// Set action attribute
        pub fn action(mut self, action: impl Into<String>) -> Self {
            self.factory.attributes.push(("action".to_string(), action.into()));
            self
        }

        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }

        /// Set number of digits to collect
        pub fn num_digits(mut self, num: impl Into<String>) -> Self {
            self.factory.attributes.push(("numDigits".to_string(), num.into()));
            self
        }
        
        /// Set timeout attribute
        pub fn timeout(mut self, timeout: usize) -> Self {
            self.factory.attributes.push(("timeout".to_string(), timeout.to_string()));
            self
        }
        
        /// Set input attribute (dtmf, speech, or dtmf speech)
        pub fn input(mut self, input: impl Into<String>) -> Self {
            self.factory.attributes.push(("input".to_string(), input.into()));
            self
        }
        
        /// Set language attribute
        pub fn language(mut self, language: impl Into<String>) -> Self {
            self.factory.attributes.push(("language".to_string(), language.into()));
            self
        }
        
        /// Set finishOnKey attribute
        pub fn finish_on_key(mut self, key: impl Into<String>) -> Self {
            self.factory.attributes.push(("finishOnKey".to_string(), key.into()));
            self
        }
        
        /// Set hints attribute for speech recognition
        pub fn hints(mut self, hints: impl Into<String>) -> Self {
            self.factory.attributes.push(("hints".to_string(), hints.into()));
            self
        }

        /// Add a Say child element
        pub fn say(mut self, say: Say) -> Self {
            self.factory.children.push(Box::new(say));
            self
        }
        
        /// Add a Play child element
        pub fn play(mut self, play: Play) -> Self {
            self.factory.children.push(Box::new(play));
            self
        }
        
        /// Add a Pause child element
        pub fn pause(mut self, pause: Pause) -> Self {
            self.factory.children.push(Box::new(pause));
            self
        }
    }

    impl TwiMLElement for Gather {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Redirect TwiML Element
    #[derive(Debug)]
    pub struct Redirect {
        factory: ElementFactory,
    }

    impl Redirect {
        /// Create a new Redirect element
        pub fn new(url: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Redirect", Some(url)),
            }
        }
        
        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }
    }

    impl TwiMLElement for Redirect {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Play TwiML Element to play audio files
    #[derive(Debug)]
    pub struct Play {
        factory: ElementFactory,
    }
    
    impl Play {
        /// Create a new Play element
        pub fn new(url: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Play", Some(url)),
            }
        }
        
        /// Set loop attribute
        pub fn loop_times(mut self, loops: usize) -> Self {
            self.factory.attributes.push(("loop".to_string(), loops.to_string()));
            self
        }
        
        /// Set digits attribute
        pub fn digits(mut self, digits: impl Into<String>) -> Self {
            self.factory.attributes.push(("digits".to_string(), digits.into()));
            self
        }
    }
    
    impl TwiMLElement for Play {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Pause TwiML Element for silent pause
    #[derive(Debug)]
    pub struct Pause {
        factory: ElementFactory,
    }
    
    impl Pause {
        /// Create a new Pause element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Pause", None::<String>),
            }
        }
        
        /// Set length attribute (seconds)
        pub fn length(mut self, seconds: usize) -> Self {
            self.factory.attributes.push(("length".to_string(), seconds.to_string()));
            self
        }
    }
    
    impl TwiMLElement for Pause {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Hangup TwiML Element to end a call
    #[derive(Debug)]
    pub struct Hangup {
        factory: ElementFactory,
    }
    
    impl Hangup {
        /// Create a new Hangup element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Hangup", None::<String>),
            }
        }
    }
    
    impl TwiMLElement for Hangup {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Record TwiML Element to record caller's voice
    #[derive(Debug)]
    pub struct Record {
        factory: ElementFactory,
    }
    
    impl Record {
        /// Create a new Record element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Record", None::<String>),
            }
        }
        
        /// Set action attribute
        pub fn action(mut self, action: impl Into<String>) -> Self {
            self.factory.attributes.push(("action".to_string(), action.into()));
            self
        }
        
        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }
        
        /// Set timeout attribute
        pub fn timeout(mut self, timeout: usize) -> Self {
            self.factory.attributes.push(("timeout".to_string(), timeout.to_string()));
            self
        }
        
        /// Set finishOnKey attribute
        pub fn finish_on_key(mut self, key: impl Into<String>) -> Self {
            self.factory.attributes.push(("finishOnKey".to_string(), key.into()));
            self
        }
        
        /// Set maxLength attribute
        pub fn max_length(mut self, seconds: usize) -> Self {
            self.factory.attributes.push(("maxLength".to_string(), seconds.to_string()));
            self
        }
        
        /// Set playBeep attribute
        pub fn play_beep(mut self, play_beep: bool) -> Self {
            self.factory.attributes.push(("playBeep".to_string(), play_beep.to_string()));
            self
        }
        
        /// Set recordingStatusCallback attribute
        pub fn recording_status_callback(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("recordingStatusCallback".to_string(), url.into()));
            self
        }
        
        /// Set recordingStatusCallbackMethod attribute
        pub fn recording_status_callback_method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("recordingStatusCallbackMethod".to_string(), method.into()));
            self
        }
        
        /// Set transcribe attribute
        pub fn transcribe(mut self, transcribe: bool) -> Self {
            self.factory.attributes.push(("transcribe".to_string(), transcribe.to_string()));
            self
        }
        
        /// Set transcribeCallback attribute
        pub fn transcribe_callback(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("transcribeCallback".to_string(), url.into()));
            self
        }
    }
    
    impl TwiMLElement for Record {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Dial TwiML Element to connect call to another phone
    #[derive(Debug)]
    pub struct Dial {
        factory: ElementFactory,
    }
    
    impl Dial {
        /// Create a new Dial element with optional number to dial
        pub fn new(number: Option<impl Into<String>>) -> Self {
            Self {
                factory: ElementFactory::new("Dial", number.map(|n| n.into())),
            }
        }
        
        /// Create a new empty Dial element without any number
        pub fn new_empty() -> Self {
            Self {
                factory: ElementFactory::new("Dial", None::<String>),
            }
        }
        
        /// Set action attribute
        pub fn action(mut self, action: impl Into<String>) -> Self {
            self.factory.attributes.push(("action".to_string(), action.into()));
            self
        }
        
        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }
        
        /// Set timeout attribute
        pub fn timeout(mut self, timeout: usize) -> Self {
            self.factory.attributes.push(("timeout".to_string(), timeout.to_string()));
            self
        }
        
        /// Set callerId attribute
        pub fn caller_id(mut self, caller_id: impl Into<String>) -> Self {
            self.factory.attributes.push(("callerId".to_string(), caller_id.into()));
            self
        }
        
        /// Set record attribute
        pub fn record(mut self, record: impl Into<String>) -> Self {
            self.factory.attributes.push(("record".to_string(), record.into()));
            self
        }
        
        /// Set Number child element
        pub fn number(mut self, number: Number) -> Self {
            self.factory.children.push(Box::new(number));
            self
        }
        
        /// Set Client child element
        pub fn client(mut self, client: Client) -> Self {
            self.factory.children.push(Box::new(client));
            self
        }
        
        /// Set Conference child element
        pub fn conference(mut self, conference: Conference) -> Self {
            self.factory.children.push(Box::new(conference));
            self
        }
        
        /// Set Sip child element
        pub fn sip(mut self, sip: Sip) -> Self {
            self.factory.children.push(Box::new(sip));
            self
        }
    }
    
    impl TwiMLElement for Dial {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Number TwiML Element noun for Dial
    #[derive(Debug)]
    pub struct Number {
        factory: ElementFactory,
    }
    
    impl Number {
        /// Create a new Number element
        pub fn new(number: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Number", Some(number)),
            }
        }
        
        /// Set sendDigits attribute
        pub fn send_digits(mut self, digits: impl Into<String>) -> Self {
            self.factory.attributes.push(("sendDigits".to_string(), digits.into()));
            self
        }
        
        /// Set url attribute
        pub fn url(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("url".to_string(), url.into()));
            self
        }
    }
    
    impl TwiMLElement for Number {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Client TwiML Element noun for Dial
    #[derive(Debug)]
    pub struct Client {
        factory: ElementFactory,
    }
    
    impl Client {
        /// Create a new Client element
        pub fn new(client_id: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Client", Some(client_id)),
            }
        }
        
        /// Set url attribute
        pub fn url(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("url".to_string(), url.into()));
            self
        }
        
        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }
        
        /// Set statusCallback attribute
        pub fn status_callback(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("statusCallback".to_string(), url.into()));
            self
        }
        
        /// Set statusCallbackMethod attribute
        pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("statusCallbackMethod".to_string(), method.into()));
            self
        }
    }
    
    impl TwiMLElement for Client {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Conference TwiML Element noun for Dial
    #[derive(Debug)]
    pub struct Conference {
        factory: ElementFactory,
    }
    
    impl Conference {
        /// Create a new Conference element
        pub fn new(room_name: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Conference", Some(room_name)),
            }
        }
        
        /// Set muted attribute
        pub fn muted(mut self, muted: bool) -> Self {
            self.factory.attributes.push(("muted".to_string(), muted.to_string()));
            self
        }
        
        /// Set startConferenceOnEnter attribute
        pub fn start_conference_on_enter(mut self, start: bool) -> Self {
            self.factory.attributes.push(("startConferenceOnEnter".to_string(), start.to_string()));
            self
        }
        
        /// Set endConferenceOnExit attribute
        pub fn end_conference_on_exit(mut self, end: bool) -> Self {
            self.factory.attributes.push(("endConferenceOnExit".to_string(), end.to_string()));
            self
        }
        
        /// Set maxParticipants attribute
        pub fn max_participants(mut self, max: usize) -> Self {
            self.factory.attributes.push(("maxParticipants".to_string(), max.to_string()));
            self
        }
        
        /// Set beep attribute
        pub fn beep(mut self, beep: bool) -> Self {
            self.factory.attributes.push(("beep".to_string(), beep.to_string()));
            self
        }
        
        /// Set record attribute
        pub fn record(mut self, record: impl Into<String>) -> Self {
            self.factory.attributes.push(("record".to_string(), record.into()));
            self
        }
    }
    
    impl TwiMLElement for Conference {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Sip TwiML Element noun for Dial
    #[derive(Debug)]
    pub struct Sip {
        factory: ElementFactory,
    }
    
    impl Sip {
        /// Create a new Sip element
        pub fn new(sip_url: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Sip", Some(sip_url)),
            }
        }
        
        /// Set username attribute
        pub fn username(mut self, username: impl Into<String>) -> Self {
            self.factory.attributes.push(("username".to_string(), username.into()));
            self
        }
        
        /// Set password attribute
        pub fn password(mut self, password: impl Into<String>) -> Self {
            self.factory.attributes.push(("password".to_string(), password.into()));
            self
        }
    }
    
    impl TwiMLElement for Sip {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// SMS TwiML Element to send text message during a call
    #[derive(Debug)]
    pub struct Sms {
        factory: ElementFactory,
    }
    
    impl Sms {
        /// Create a new Sms element
        pub fn new(message: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Sms", Some(message)),
            }
        }
        
        /// Set to attribute
        pub fn to(mut self, to: impl Into<String>) -> Self {
            self.factory.attributes.push(("to".to_string(), to.into()));
            self
        }
        
        /// Set from attribute
        pub fn from(mut self, from: impl Into<String>) -> Self {
            self.factory.attributes.push(("from".to_string(), from.into()));
            self
        }
        
        /// Set action attribute
        pub fn action(mut self, action: impl Into<String>) -> Self {
            self.factory.attributes.push(("action".to_string(), action.into()));
            self
        }
        
        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }
    }
    
    impl TwiMLElement for Sms {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Body TwiML Element used within Message
    #[derive(Debug)]
    pub struct Body {
        factory: ElementFactory,
    }
    
    impl Body {
        /// Create a new Body element
        pub fn new(text: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Body", Some(text)),
            }
        }
    }
    
    impl TwiMLElement for Body {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Message TwiML Element for sending messages
    #[derive(Debug)]
    pub struct Message {
        factory: ElementFactory,
    }
    
    impl Message {
        /// Create a new Message element with optional text
        pub fn new(text: Option<impl Into<String>>) -> Self {
            Self {
                factory: ElementFactory::new("Message", text.map(|t| t.into())),
            }
        }
        
        /// Create a new Message element with no text content
        pub fn new_empty() -> Self {
            Self {
                factory: ElementFactory::new("Message", None::<String>),
            }
        }
        
        /// Create a new Message element with direct text content
        pub fn new_with_text(text: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Message", Some(text.into())),
            }
        }
        
        /// Set to attribute
        pub fn to(mut self, to: impl Into<String>) -> Self {
            self.factory.attributes.push(("to".to_string(), to.into()));
            self
        }
        
        /// Set from attribute
        pub fn from(mut self, from: impl Into<String>) -> Self {
            self.factory.attributes.push(("from".to_string(), from.into()));
            self
        }
        
        /// Set action attribute
        pub fn action(mut self, action: impl Into<String>) -> Self {
            self.factory.attributes.push(("action".to_string(), action.into()));
            self
        }
        
        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }
        
        /// Set statusCallback attribute
        pub fn status_callback(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("statusCallback".to_string(), url.into()));
            self
        }
        
        /// Add a Body child element
        pub fn body(mut self, body: Body) -> Self {
            self.factory.children.push(Box::new(body));
            self
        }
    }
    
    impl TwiMLElement for Message {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Enqueue TwiML Element to add call to a queue
    #[derive(Debug)]
    pub struct Enqueue {
        factory: ElementFactory,
    }
    
    impl Enqueue {
        /// Create a new Enqueue element
        pub fn new(queue_name: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Enqueue", Some(queue_name)),
            }
        }
        
        /// Set action attribute
        pub fn action(mut self, action: impl Into<String>) -> Self {
            self.factory.attributes.push(("action".to_string(), action.into()));
            self
        }
        
        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }
        
        /// Set waitUrl attribute
        pub fn wait_url(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("waitUrl".to_string(), url.into()));
            self
        }
        
        /// Set waitUrlMethod attribute
        pub fn wait_url_method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("waitUrlMethod".to_string(), method.into()));
            self
        }
    }
    
    impl TwiMLElement for Enqueue {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
    
    /// Leave TwiML Element to exit a queue
    #[derive(Debug)]
    pub struct Leave {
        factory: ElementFactory,
    }
    
    impl Leave {
        /// Create a new Leave element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Leave", None::<String>),
            }
        }
    }
    
    impl TwiMLElement for Leave {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Response TwiML Element - the root element
    #[derive(Debug)]
    pub struct Response {
        factory: ElementFactory,
    }

    impl Response {
        /// Create a new Response element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Response", None::<String>),
            }
        }

        /// Add a Say child element
        pub fn say(mut self, say: Say) -> Self {
            self.factory.children.push(Box::new(say));
            self
        }

        /// Add a Gather child element
        pub fn gather(mut self, gather: Gather) -> Self {
            self.factory.children.push(Box::new(gather));
            self
        }

        /// Add a Redirect child element
        pub fn redirect(mut self, redirect: Redirect) -> Self {
            self.factory.children.push(Box::new(redirect));
            self
        }
        
        /// Add a Play child element
        pub fn play(mut self, play: Play) -> Self {
            self.factory.children.push(Box::new(play));
            self
        }
        
        /// Add a Pause child element
        pub fn pause(mut self, pause: Pause) -> Self {
            self.factory.children.push(Box::new(pause));
            self
        }
        
        /// Add a Hangup child element
        pub fn hangup(mut self) -> Self {
            self.factory.children.push(Box::new(Hangup::new()));
            self
        }
        
        /// Add a Record child element
        pub fn record(mut self, record: Record) -> Self {
            self.factory.children.push(Box::new(record));
            self
        }
        
        /// Add a Dial child element
        pub fn dial(mut self, dial: Dial) -> Self {
            self.factory.children.push(Box::new(dial));
            self
        }
        
        /// Add an Sms child element
        pub fn sms(mut self, sms: Sms) -> Self {
            self.factory.children.push(Box::new(sms));
            self
        }
        
        /// Add a Message child element
        pub fn message(mut self, message: Message) -> Self {
            self.factory.children.push(Box::new(message));
            self
        }
        
        /// Add an Enqueue child element
        pub fn enqueue(mut self, enqueue: Enqueue) -> Self {
            self.factory.children.push(Box::new(enqueue));
            self
        }
        
        /// Add a Leave child element
        pub fn leave(mut self) -> Self {
            self.factory.children.push(Box::new(Leave::new()));
            self
        }
        
        /// Add direct text to the response
        pub fn text(mut self, text: impl Into<String>) -> Self {
            self.factory.text = Some(text.into());
            self
        }
    }

    impl TwiMLElement for Response {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }
}

// Export all elements from the module
pub use elements::*;

// Example usage and test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_string_conversion() {
        let response = Response::new()
            .say(Say::new("Welcome to our service"))
            .redirect(Redirect::new("/next-step"));
        
        // Convert to String using to_xml_string()
        let xml_string = response.to_xml_string();
        assert!(xml_string.contains("Welcome to our service"));
        assert!(xml_string.contains("<Redirect>/next-step</Redirect>"));
    }
    
    #[test]
    fn test_complex_call_flow() {
        let response = Response::new()
            .say(Say::new("Welcome to our service").voice("alice").language("en-US"))
            .gather(
                Gather::new()
                    .num_digits("1")
                    .timeout(5)
                    .action("/process-selection")
                    .method("POST")
                    .hints("sales support billing")
                    .say(Say::new("Press 1 for sales, press 2 for support"))
            )
            .dial(
                Dial::new(None::<String>)
                    .timeout(10)
                    .action("/handle-dial")
                    .number(Number::new("+1234567890"))
            );
        
        let xml_string = response.to_xml_string();
        assert!(xml_string.contains("Welcome to our service"));
        assert!(xml_string.contains("Press 1 for sales"));
        assert!(xml_string.contains("<Number>+1234567890</Number>"));
    }
    
    #[test]
    fn test_conference_call() {
        let response = Response::new()
            .say(Say::new("You are about to join the conference"))
            .dial(
                Dial::new(None::<String>)
                    .conference(
                        Conference::new("MyRoom")
                            .muted(false)
                            .beep(true)
                            .start_conference_on_enter(true)
                            .end_conference_on_exit(false)
                            .max_participants(10)
                    )
            );
        
        let xml_string = response.to_xml_string();
        assert!(xml_string.contains("You are about to join the conference"));
        assert!(xml_string.contains("<Conference"));
        assert!(xml_string.contains("MyRoom"));
        assert!(xml_string.contains("startConferenceOnEnter=\"true\""));
        assert!(xml_string.contains("beep=\"true\""));
    }
    
    #[test]
    fn test_recording() {
        let response = Response::new()
            .say(Say::new("Please leave a message after the tone"))
            .record(
                Record::new()
                    .timeout(10)
                    .max_length(60)
                    .action("/handle-recording")
                    .play_beep(true)
            )
            .say(Say::new("Thank you for your message"));
        
        let xml_string = response.to_xml_string();
        assert!(xml_string.contains("Please leave a message"));
        assert!(xml_string.contains("<Record"));
        assert!(xml_string.contains("maxLength=\"60\""));
        assert!(xml_string.contains("playBeep=\"true\""));
    }
    
    #[test]
    fn test_text_in_response() {
        let response = Response::new()
            .text("This is direct text in the response");
        
        let xml_string = response.to_xml_string();
        assert!(xml_string.contains("This is direct text in the response"));
    }
    
    #[test]
    fn test_message_with_body() {
        let response = Response::new()
            .message(
                Message::new_empty()
                    .to("+12345678900")
                    .from("+10987654321")
                    .body(Body::new("Hello, this is a test message"))
            );
        
        let xml_string = response.to_xml_string();
        assert!(xml_string.contains("<Message"));
        assert!(xml_string.contains("to=\"+12345678900\""));
        assert!(xml_string.contains("<Body>Hello, this is a test message</Body>"));
    }
}

// Recommended Cargo.toml dependencies:
// [dependencies]
// xml-builder = "0.5.1"
