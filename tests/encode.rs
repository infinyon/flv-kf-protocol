use kf_protocol::derive::Encode;
use kf_protocol::Encoder;
use kf_protocol::EncoderVarInt;

#[derive(Encode, Default, Debug)]
pub struct SimpleRecord {
    #[varint]
    len: i64,
    attributes: i8,
}

#[derive(Encode, Default, Debug)]
pub struct RecordSet {
    records: Vec<SimpleRecord>,
}

impl RecordSet {
    fn add_record(&mut self, record: SimpleRecord) {
        (&mut self.records).push(record);
    }
}

#[test]
fn test_encode_recordset() {
    let mut recordset = RecordSet::default();
    let mut record = SimpleRecord::default();
    record.attributes = 10;
    record.len = 4;
    recordset.add_record(record);

    let mut src = vec![];
    let result = recordset.encode(&mut src, 0);
    assert!(result.is_ok());
    assert_eq!(src.len(), 6);
    assert_eq!(src[5], 0x0a);
    assert_eq!(recordset.write_size(0), 6);
}
