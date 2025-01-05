use std::io::Read;

#[allow(unused_variables)]
fn read_version(transaction_bytes: &[u8]) -> u32 {
    // declare buffer of size of length 4
    let mut buffer = [0; 4];
    // read into the buffer
    transaction_bytes.read(&mut buffer).unwrap();
    // converting bytes to integer - taking note that bitcoing is little endian on bytes
    u32::from_le_bytes(buffer)
}
#[allow(unused_variables)]
fn main() {
    let transaction_hex ="0200000000010175a242c76bacc05a763a0bd61bd2a645eff7d7420a29740e9fe14887085850d50100000000fdffffff024a01000000000000160014bd045a5f325b46d59b501199b1f4f376fc28ba661bda07000000000016001469a12063607de9f7a1d9ef92e2e3cb5a20361d7903406a03aed86aa774f85d53840d016482ebd68cbb6d5e2d89cad0fb2a36bb8e88de42c04b9ef0cc8389b13fff8dbfe490320857d62ee3d93a0f44d7279b1f561dff6d2058687191b2eed5c18c01653537af5824f684bb1a14004bcb92662cb67e1cbe10ac0063036f726401010a746578742f706c61696e00357b2270223a226272632d3230222c226f70223a226d696e74222c227469636b223a22cf80c2ad222c22616d74223a2231303030227d6821c158687191b2eed5c18c01653537af5824f684bb1a14004bcb92662cb67e1cbe1000000000";

    // decode to raw bytes - this is a vector
    let transaction_bytes = hex::decode(transaction_hex).unwrap();

    // conver the decoded bytes to slice
    let mut byte_slice = transaction_bytes.as_slice();

    // call the read version function
    let version = read_version(byte_slice);

    println!("version: {}!", version);
}
