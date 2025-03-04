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

mod elements {
    use super::*;
    use xml_builder::{XMLElement};

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

    /// Response TwiML Element - The root element for TwiML documents
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

        /// Add a Play child element
        pub fn play(mut self, play: Play) -> Self {
            self.factory.children.push(Box::new(play));
            self
        }

        /// Add a Gather child element
        pub fn gather(mut self, gather: Gather) -> Self {
            self.factory.children.push(Box::new(gather));
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

        /// Add a Message child element
        pub fn message(mut self, message: Message) -> Self {
            self.factory.children.push(Box::new(message));
            self
        }

        /// Add a Redirect child element
        pub fn redirect(mut self, url: impl Into<String>) -> Self {
            self.factory.children.push(Box::new(Redirect::new(url)));
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

        /// Add a Reject child element
        pub fn reject(mut self, reject: Reject) -> Self {
            self.factory.children.push(Box::new(reject));
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

        /// Add a Connect child element
        pub fn connect(mut self, connect: Connect) -> Self {
            self.factory.children.push(Box::new(connect));
            self
        }

        /// Add a Pay child element
        pub fn pay(mut self, pay: Pay) -> Self {
            self.factory.children.push(Box::new(pay));
            self
        }

        /// Add a Refer child element
        pub fn refer(mut self, refer: Refer) -> Self {
            self.factory.children.push(Box::new(refer));
            self
        }

        /// Add a Stream child element
        pub fn stream(mut self, stream: Stream) -> Self {
            self.factory.children.push(Box::new(stream));
            self
        }
    }

    impl TwiMLElement for Response {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
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
        pub fn loop_times(mut self, count: usize) -> Self {
            self.factory.attributes.push(("loop".to_string(), count.to_string()));
            self
        }
    }

    impl TwiMLElement for Say {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Play TwiML Element for playing audio
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
        pub fn loop_times(mut self, count: usize) -> Self {
            self.factory.attributes.push(("loop".to_string(), count.to_string()));
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

        /// Set finishOnKey attribute
        pub fn finish_on_key(mut self, key: impl Into<String>) -> Self {
            self.factory.attributes.push(("finishOnKey".to_string(), key.into()));
            self
        }

        /// Set input attribute (dtmf, speech, or dtmf speech)
        pub fn input(mut self, input: impl Into<String>) -> Self {
            self.factory.attributes.push(("input".to_string(), input.into()));
            self
        }

        /// Set language attribute for speech recognition
        pub fn language(mut self, language: impl Into<String>) -> Self {
            self.factory.attributes.push(("language".to_string(), language.into()));
            self
        }

        /// Set hints attribute for speech recognition
        pub fn hints(mut self, hints: impl Into<String>) -> Self {
            self.factory.attributes.push(("hints".to_string(), hints.into()));
            self
        }

        /// Set profanityFilter attribute for speech recognition
        pub fn profanity_filter(mut self, filter: bool) -> Self {
            self.factory.attributes.push(("profanityFilter".to_string(), filter.to_string()));
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

    /// Record TwiML Element for recording caller's voice
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

        /// Set finish on key attribute
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

    /// Dial TwiML Element for connecting the call to another phone number
    #[derive(Debug)]
    pub struct Dial {
        factory: ElementFactory,
    }

    impl Dial {
        /// Create a new Dial element with an optional destination
        pub fn new(destination: Option<impl Into<String>>) -> Self {
            Self {
                factory: ElementFactory::new("Dial", destination.map(|d| d.into())),
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

        /// Set hangupOnStar attribute
        pub fn hangup_on_star(mut self, hangup: bool) -> Self {
            self.factory.attributes.push(("hangupOnStar".to_string(), hangup.to_string()));
            self
        }

        /// Set timeLimit attribute
        pub fn time_limit(mut self, seconds: usize) -> Self {
            self.factory.attributes.push(("timeLimit".to_string(), seconds.to_string()));
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

        /// Set trim attribute
        pub fn trim(mut self, trim: impl Into<String>) -> Self {
            self.factory.attributes.push(("trim".to_string(), trim.into()));
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

        /// Set answerOnBridge attribute
        pub fn answer_on_bridge(mut self, answer: bool) -> Self {
            self.factory.attributes.push(("answerOnBridge".to_string(), answer.to_string()));
            self
        }

        /// Set ringTone attribute
        pub fn ring_tone(mut self, tone: impl Into<String>) -> Self {
            self.factory.attributes.push(("ringTone".to_string(), tone.into()));
            self
        }

        /// Add a Number child element
        pub fn number(mut self, number: Number) -> Self {
            self.factory.children.push(Box::new(number));
            self
        }

        /// Add a Client child element
        pub fn client(mut self, client: Client) -> Self {
            self.factory.children.push(Box::new(client));
            self
        }

        /// Add a Conference child element
        pub fn conference(mut self, conference: Conference) -> Self {
            self.factory.children.push(Box::new(conference));
            self
        }

        /// Add a Sip child element
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

    /// Number TwiML Element for specifying a phone number in a Dial
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

        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.factory.attributes.push(("method".to_string(), method.into()));
            self
        }

        /// Set statusCallbackEvent attribute
        pub fn status_callback_event(mut self, events: impl Into<String>) -> Self {
            self.factory.attributes.push(("statusCallbackEvent".to_string(), events.into()));
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

    impl TwiMLElement for Number {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Client TwiML Element for specifying a client identifier in a Dial
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

        /// Set statusCallbackEvent attribute
        pub fn status_callback_event(mut self, events: impl Into<String>) -> Self {
            self.factory.attributes.push(("statusCallbackEvent".to_string(), events.into()));
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

    /// Conference TwiML Element for specifying a conference in a Dial
    #[derive(Debug)]
    pub struct Conference {
        factory: ElementFactory,
    }

    impl Conference {
        /// Create a new Conference element
        pub fn new(conference_name: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Conference", Some(conference_name)),
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
        pub fn beep(mut self, beep: impl Into<String>) -> Self {
            self.factory.attributes.push(("beep".to_string(), beep.into()));
            self
        }

        /// Set record attribute
        pub fn record(mut self, record: impl Into<String>) -> Self {
            self.factory.attributes.push(("record".to_string(), record.into()));
            self
        }

        /// Set trim attribute
        pub fn trim(mut self, trim: impl Into<String>) -> Self {
            self.factory.attributes.push(("trim".to_string(), trim.into()));
            self
        }

        /// Set statusCallbackEvent attribute
        pub fn status_callback_event(mut self, events: impl Into<String>) -> Self {
            self.factory.attributes.push(("statusCallbackEvent".to_string(), events.into()));
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

        /// Set region attribute
        pub fn region(mut self, region: impl Into<String>) -> Self {
            self.factory.attributes.push(("region".to_string(), region.into()));
            self
        }
    }

    impl TwiMLElement for Conference {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Sip TwiML Element for SIP endpoints in a Dial
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

        /// Set statusCallbackEvent attribute
        pub fn status_callback_event(mut self, events: impl Into<String>) -> Self {
            self.factory.attributes.push(("statusCallbackEvent".to_string(), events.into()));
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

    impl TwiMLElement for Sip {
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
        /// Create a new Message element
        pub fn new(message: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Message", Some(message)),
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

        /// Add a Media child element
        pub fn media(mut self, media: Media) -> Self {
            self.factory.children.push(Box::new(media));
            self
        }
    }

    impl TwiMLElement for Message {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Body TwiML Element for Message content
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

    /// Media TwiML Element for Message media content
    #[derive(Debug)]
    pub struct Media {
        factory: ElementFactory,
    }

    impl Media {
        /// Create a new Media element
        pub fn new(url: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Media", Some(url)),
            }
        }
    }

    impl TwiMLElement for Media {
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

    /// Pause TwiML Element
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

        /// Set length attribute (in seconds)
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

    /// Hangup TwiML Element
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

    /// Reject TwiML Element
    #[derive(Debug)]
    pub struct Reject {
        factory: ElementFactory,
    }

    impl Reject {
        /// Create a new Reject element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Reject", None::<String>),
            }
        }

        /// Set reason attribute
        pub fn reason(mut self, reason: impl Into<String>) -> Self {
            self.factory.attributes.push(("reason".to_string(), reason.into()));
            self
        }
    }

    impl TwiMLElement for Reject {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Enqueue TwiML Element
    #[derive(Debug)]
    pub struct Enqueue {
        factory: ElementFactory,
    }

    impl Enqueue {
        /// Create a new Enqueue element
        pub fn new(queue_name: Option<impl Into<String>>) -> Self {
            Self {
                factory: ElementFactory::new("Enqueue", queue_name.map(|q| q.into())),
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

        /// Set workflowSid attribute
        pub fn workflow_sid(mut self, sid: impl Into<String>) -> Self {
            self.factory.attributes.push(("workflowSid".to_string(), sid.into()));
            self
        }

        /// Add a Task child element
        pub fn task(mut self, task: Task) -> Self {
            self.factory.children.push(Box::new(task));
            self
        }
    }

    impl TwiMLElement for Enqueue {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Task TwiML Element for Enqueue
    #[derive(Debug)]
    pub struct Task {
        factory: ElementFactory,
    }

    impl Task {
        /// Create a new Task element
        pub fn new(task_json: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Task", Some(task_json)),
            }
        }

        /// Set priority attribute
        pub fn priority(mut self, priority: usize) -> Self {
            self.factory.attributes.push(("priority".to_string(), priority.to_string()));
            self
        }

        /// Set timeout attribute
        pub fn timeout(mut self, timeout: usize) -> Self {
            self.factory.attributes.push(("timeout".to_string(), timeout.to_string()));
            self
        }
    }

    impl TwiMLElement for Task {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Leave TwiML Element
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

    /// Connect TwiML Element
    #[derive(Debug)]
    pub struct Connect {
        factory: ElementFactory,
    }

    impl Connect {
        /// Create a new Connect element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Connect", None::<String>),
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

        /// Add a Room child element
        pub fn room(mut self, room: Room) -> Self {
            self.factory.children.push(Box::new(room));
            self
        }

        /// Add a Stream child element
        pub fn stream(mut self, stream: Stream) -> Self {
            self.factory.children.push(Box::new(stream));
            self
        }
    }

    impl TwiMLElement for Connect {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Room TwiML Element for Connect
    #[derive(Debug)]
    pub struct Room {
        factory: ElementFactory,
    }

    impl Room {
        /// Create a new Room element
        pub fn new(room_name: impl Into<String>) -> Self {
            Self {
                factory: ElementFactory::new("Room", Some(room_name)),
            }
        }

        /// Set participantIdentity attribute
        pub fn participant_identity(mut self, identity: impl Into<String>) -> Self {
            self.factory.attributes.push(("participantIdentity".to_string(), identity.into()));
            self
        }
    }

    impl TwiMLElement for Room {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Stream TwiML Element
    #[derive(Debug)]
    pub struct Stream {
        factory: ElementFactory,
    }

    impl Stream {
        /// Create a new Stream element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Stream", None::<String>),
            }
        }

        /// Set url attribute
        pub fn url(mut self, url: impl Into<String>) -> Self {
            self.factory.attributes.push(("url".to_string(), url.into()));
            self
        }

        /// Set name attribute
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.factory.attributes.push(("name".to_string(), name.into()));
            self
        }

        /// Set value attribute
        pub fn value(mut self, value: impl Into<String>) -> Self {
            self.factory.attributes.push(("value".to_string(), value.into()));
            self
        }
    }

    impl TwiMLElement for Parameter {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Prompt TwiML Element for Pay
    #[derive(Debug)]
    pub struct Prompt {
        factory: ElementFactory,
    }

    impl Prompt {
        /// Create a new Prompt element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Prompt", None::<String>),
            }
        }

        /// Set for attribute
        pub fn for_attribute(mut self, for_value: impl Into<String>) -> Self {
            self.factory.attributes.push(("for".to_string(), for_value.into()));
            self
        }

        /// Set attempt attribute
        pub fn attempt(mut self, attempt: usize) -> Self {
            self.factory.attributes.push(("attempt".to_string(), attempt.to_string()));
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

    impl TwiMLElement for Prompt {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Refer TwiML Element
    #[derive(Debug)]
    pub struct Refer {
        factory: ElementFactory,
    }

    impl Refer {
        /// Create a new Refer element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Refer", None::<String>),
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

        /// Add a Sip child element
        pub fn sip(mut self, sip: ReferSip) -> Self {
            self.factory.children.push(Box::new(sip));
            self
        }
    }

    impl TwiMLElement for Refer {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// ReferSip TwiML Element for Refer
    #[derive(Debug)]
    pub struct ReferSip {
        factory: ElementFactory,
    }

    impl ReferSip {
        /// Create a new ReferSip element
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
    }

    impl TwiMLElement for ReferSip {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// ReferenceIdentity TwiML Element for Refer
    #[derive(Debug)]
    pub struct ReferenceIdentity {
        factory: ElementFactory,
    }

    impl ReferenceIdentity {
        /// Create a new ReferenceIdentity element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Identity", None::<String>),
            }
        }

        /// Set customer name attribute
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.factory.attributes.push(("name".to_string(), name.into()));
            self
        }
        
        /// Set parameter attribute
        pub fn parameter(mut self, parameter: Parameter) -> Self {
            self.factory.children.push(Box::new(parameter));
            self
        }
    }

    impl TwiMLElement for ReferenceIdentity {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    impl TwiMLElement for Stream {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Pay TwiML Element
    #[derive(Debug)]
    pub struct Pay {
        factory: ElementFactory,
    }

    impl Pay {
        /// Create a new Pay element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Pay", None::<String>),
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

        /// Set chargeAmount attribute
        pub fn charge_amount(mut self, amount: impl Into<String>) -> Self {
            self.factory.attributes.push(("chargeAmount".to_string(), amount.into()));
            self
        }

        /// Set paymentConnector attribute
        pub fn payment_connector(mut self, connector: impl Into<String>) -> Self {
            self.factory.attributes.push(("paymentConnector".to_string(), connector.into()));
            self
        }

        /// Set tokenType attribute
        pub fn token_type(mut self, token_type: impl Into<String>) -> Self {
            self.factory.attributes.push(("tokenType".to_string(), token_type.into()));
            self
        }

        /// Set currency attribute
        pub fn currency(mut self, currency: impl Into<String>) -> Self {
            self.factory.attributes.push(("currency".to_string(), currency.into()));
            self
        }

        /// Set description attribute
        pub fn description(mut self, description: impl Into<String>) -> Self {
            self.factory.attributes.push(("description".to_string(), description.into()));
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

        /// Set timeout attribute
        pub fn timeout(mut self, timeout: usize) -> Self {
            self.factory.attributes.push(("timeout".to_string(), timeout.to_string()));
            self
        }

        /// Add a Parameter child element
        pub fn parameter(mut self, parameter: Parameter) -> Self {
            self.factory.children.push(Box::new(parameter));
            self
        }

        /// Add a Prompt child element
        pub fn prompt(mut self, prompt: Prompt) -> Self {
            self.factory.children.push(Box::new(prompt));
            self
        }
    }

    impl TwiMLElement for Pay {
        fn to_xml(&self) -> XMLElement {
            self.factory.to_xml()
        }
    }

    /// Parameter TwiML Element for Pay
    #[derive(Debug)]
    pub struct Parameter {
        factory: ElementFactory,
    }

    impl Parameter {
        /// Create a new Parameter element
        pub fn new() -> Self {
            Self {
                factory: ElementFactory::new("Parameter", None::<String>),
            }
        }

        /// Set name attribute
        pub fn name(mut self, name: impl Into