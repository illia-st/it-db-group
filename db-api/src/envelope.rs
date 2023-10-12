use ion_rs;
use ion_rs::element::writer::TextKind;
use ion_rs::IonWriter;
use ion_rs::IonReader;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Envelope {
    command_type: String,
    data: Vec<u8>,
}

impl Envelope {
    pub fn new(envelope_type: &str, data: &[u8]) -> Self {
        Envelope {
            command_type: envelope_type.into(),
            data: data.into()
        }
    }

    pub fn get_type (&self) -> &str {
        &self.command_type
    }

    pub fn get_data (&self) -> &[u8] {
        &self.data
    }
}

impl crate::Encoder for Envelope {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact);

        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("type");
        writer.write_string(&self.command_type).unwrap();

        writer.set_field_name("data");
        writer.write_blob(&self.data).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().to_owned()
    }
}

impl crate::Decoder for Envelope {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let envelope_type = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let data = binding.as_slice();

        Envelope::new(
            envelope_type,
            data,
        )
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use crate::Decoder;
    use crate::Encoder;

    use super::Envelope;

    #[test]
    fn reader_correctly_read_encoded_envelope() {
        const ENVELOPE_TYPE: &str = "ENVELOPE_TYPE";
        const ENVELOPE_DATA: &[u8] = "ENVELOPE_DATA".as_bytes();
        let envelope = Envelope::new(ENVELOPE_TYPE, ENVELOPE_DATA);

        let mut binary_user_reader = ReaderBuilder::new().build(envelope.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("type", binary_user_reader.field_name().unwrap());
        assert_eq!(ENVELOPE_TYPE, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("data", binary_user_reader.field_name().unwrap());
        assert_eq!(ENVELOPE_DATA, binary_user_reader.read_blob().unwrap().as_slice());
    }

    #[test]
    fn endec_envelope() {
        const ENVELOPE_TYPE: &str = "ENVELOPE_TYPE";
        const ENVELOPE_DATA: &[u8] = "ENVELOPE_DATA".as_bytes();
        let envelope = Envelope::new(ENVELOPE_TYPE, ENVELOPE_DATA);
        assert_eq!(envelope, Envelope::decode(&envelope.encode()));
    }
}