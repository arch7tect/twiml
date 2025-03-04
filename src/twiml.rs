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

    /// Say TwiML Element for text-to-speech
    #[derive(Debug, Clone)]
    pub struct Say {
        text: String,
        attributes: Vec<(String, String)>,
    }

    impl Say {
        /// Create a new Say element
        pub fn new(text: impl Into<String>) -> Self {
            Self {
                text: text.into(),
                attributes: Vec::new(),
            }
        }

        /// Set voice attribute
        pub fn voice(mut self, voice: impl Into<String>) -> Self {
            self.attributes.push(("voice".to_string(), voice.into()));
            self
        }

        /// Set language attribute
        pub fn language(mut self, language: impl Into<String>) -> Self {
            self.attributes.push(("language".to_string(), language.into()));
            self
        }
    }

    impl super::TwiMLElement for Say {
        fn to_xml(&self) -> XMLElement {
            let mut elem = XMLElement::new("Say");
            
            // Add attributes
            for (key, value) in &self.attributes {
                elem.add_attribute(key, value);
            }
            
            // Add text (use owned String)
            elem.add_text(self.text.clone()).unwrap();
            
            elem
        }
    }

    /// Gather TwiML Element for collecting user input
    #[derive(Debug)]
    pub struct Gather {
        attributes: Vec<(String, String)>,
        children: Vec<Box<dyn super::TwiMLElement>>,
    }

    impl Gather {
        /// Create a new Gather element
        pub fn new() -> Self {
            Self {
                attributes: Vec::new(),
                children: Vec::new(),
            }
        }

        /// Set action attribute
        pub fn action(mut self, action: impl Into<String>) -> Self {
            self.attributes.push(("action".to_string(), action.into()));
            self
        }

        /// Set method attribute
        pub fn method(mut self, method: impl Into<String>) -> Self {
            self.attributes.push(("method".to_string(), method.into()));
            self
        }

        /// Set number of digits to collect
        pub fn num_digits(mut self, num: impl Into<String>) -> Self {
            self.attributes.push(("numDigits".to_string(), num.into()));
            self
        }

        /// Add a Say child element
        pub fn say(mut self, say: Say) -> Self {
            self.children.push(Box::new(say));
            self
        }
    }

    impl super::TwiMLElement for Gather {
        fn to_xml(&self) -> XMLElement {
            let mut elem = XMLElement::new("Gather");
            
            // Add attributes
            for (key, value) in &self.attributes {
                elem.add_attribute(key, value);
            }
            
            // Add children
            for child in &self.children {
                elem.add_child(child.to_xml()).unwrap();
            }
            
            elem
        }
    }

    /// Redirect TwiML Element
    #[derive(Debug, Clone)]
    pub struct Redirect {
        url: String,
    }

    impl Redirect {
        /// Create a new Redirect element
        pub fn new(url: impl Into<String>) -> Self {
            Self { url: url.into() }
        }
    }

    impl super::TwiMLElement for Redirect {
        fn to_xml(&self) -> XMLElement {
            let mut elem = XMLElement::new("Redirect");
            // Use cloned owned String
            elem.add_text(self.url.clone()).unwrap();
            elem
        }
    }

    /// Response TwiML Element
    #[derive(Debug)]
    pub struct Response {
        children: Vec<Box<dyn super::TwiMLElement>>,
    }

    impl Response {
        /// Create a new Response element
        pub fn new() -> Self {
            Self { children: Vec::new() }
        }

        /// Add a Say child element
        pub fn say(mut self, say: Say) -> Self {
            self.children.push(Box::new(say));
            self
        }

        /// Add a Gather child element
        pub fn gather(mut self, gather: Gather) -> Self {
            self.children.push(Box::new(gather));
            self
        }

        /// Add a Redirect child element
        pub fn redirect(mut self, url: impl Into<String>) -> Self {
            self.children.push(Box::new(Redirect::new(url)));
            self
        }
    }

    impl super::TwiMLElement for Response {
        fn to_xml(&self) -> XMLElement {
            let mut elem = XMLElement::new("Response");
            
            // Add children
            for child in &self.children {
                elem.add_child(child.to_xml()).unwrap();
            }
            
            elem
        }
    }
}

pub use elements::*;

// Example usage and test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_string_conversion() {
        let response = Response::new()
            .say(Say::new("Welcome to our service"))
            .redirect("/next-step");
        
        // Convert to String using to_xml_string()
        let xml_string = response.to_xml_string();
        assert!(xml_string.contains("Welcome to our service"));
        assert!(xml_string.contains("<Redirect>/next-step</Redirect>"));
    }
}

// Recommended Cargo.toml dependencies:
// [dependencies]
// xml-builder = "0.5.1"
