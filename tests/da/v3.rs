use hacc::traits::TryRead;
use hacc::{Da, DaHeader, DaHeaderVersion, DaVersion};
use include_bytes_aligned::include_bytes_aligned;

const DA_V3: &[u8] = include_bytes_aligned!(4, "../files/DA_V3.bin");
const DA_DESC: &str = "MTK_AllInOne_DA_v7.1224.0.0.sn58";

#[test]
fn da_v3_parse() {
    let da = Da::try_read(DA_V3).expect("Failed to read DA V3 header");

    assert_eq!(da.header().id(), "MTK_DOWNLOAD_AGENT");
    assert_eq!(da.header().version(), DaHeaderVersion::V3);
    assert_eq!(da.header().magic(), DaHeader::DA_MAGIC);
    assert_eq!(da.header().desc(), DA_DESC);
    assert_eq!(da.entries().count(), 5);

    for entry in da.entries() {
        assert_eq!(entry.version(), DaVersion::V3);
    }
}
