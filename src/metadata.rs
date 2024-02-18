use crate::error::OpusSourceError;
use byteorder::{ByteOrder, LittleEndian};

pub struct OpusMeta {
    pub sample_rate: u32,
    pub channel_count: u8,
    pub preskip: u16,
    pub output_gain: i16,
    pub num_frames: Option<usize>,
}

impl OpusMeta {
    pub fn with_headers(
        id_header: Vec<u8>,
        comment_header: Vec<u8>,
    ) -> Result<Self, OpusSourceError> {
        //println!("First packet: {:#X?}", id_header);
        let magic = String::from_utf8((&id_header[0..8]).to_vec()).unwrap();
        if !magic.eq("OpusHead") {
            return Err(OpusSourceError::InvalidHeaderData);
        }

        let _version = &id_header[8];
        //println!("Version: {}", version);
        let channels = &id_header[9];
        //println!("Channels: {}", channels);
        let preskip = &id_header[10..12];
        let preskip = LittleEndian::read_u16(&preskip);
        //println!("Pre-Skip: {}", preskip);
        let sr = &id_header[12..16];
        let _pre_enc_sample_rate = LittleEndian::read_u32(&sr);
        //println!("Original Sample Rate: {}", pre_enc_sample_rate);
        let og = &id_header[16..18];
        let output_gain = LittleEndian::read_i16(&og);
        //println!("Output Gain: {}", output_gain);
        let _channel_mapping_family = &id_header[18];
        //println!("Channel Mapping Family: {:?}", channel_mapping_family);
        // If family is non-zero then there can be more to read

        //println!("Second packet: {:#X?}", t);
        let magic = String::from_utf8((&comment_header[0..8]).to_vec()).unwrap();
        if !magic.eq("OpusTags") {
            return Err(OpusSourceError::InvalidHeaderData);
        }

        let vs_len = &comment_header[8..12];
        let vs_len = LittleEndian::read_u32(vs_len);
        //println!("Vendor String Len: {}", vs_len);
        let vstring = &comment_header[12..12 + vs_len as usize];
        let _vstring = String::from_utf8(vstring.to_vec()).unwrap();
        //println!("Vendor String: {:?}", vstring);
        let nts = 12 + vs_len as usize;
        let num_tags = &comment_header[nts..nts + 4];
        let _num_tags = LittleEndian::read_u32(&num_tags);
        //println!("Number of Tags: {}", num_tags);
        // Pull 32bit unsigned length then matching range for utf8 text iteratively for all tags

        Ok(Self {
            sample_rate: 48_000,
            channel_count: *channels,
            preskip,
            output_gain,
            num_frames: None,
        })
    }
}
