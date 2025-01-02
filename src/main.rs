use clap::Parser;
use crc::{Crc, CRC_32_ISCSI};
use dh::{recommended::*, Rw};

const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

fn fix_hash(reader: &mut dyn Rw, offset: u64, size: u64) {
    let hash = CRC32.checksum(&reader.read_bytes_at(offset + 4, size).unwrap());
    reader.write_u32le_at(offset, hash).unwrap();
}

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();

    let mut reader = dh::file::open_rw(args.path).unwrap();

    fix_hash(&mut reader, 0x80, 0x1c);
    for i in 0..4 {
        fix_hash(&mut reader, 0xa0 + (0xa480 * i), 0x6b84);
        fix_hash(&mut reader, 0xa0 + (0xa480 * i) + 0x6b88, 0x38f4);
    }

    fix_hash(&mut reader, 0x292a0, 0x22bc8);
    fix_hash(&mut reader, 0x4be80, 0x44b8);
    fix_hash(&mut reader, 0x53424, 0x1e4d8);
    fix_hash(&mut reader, 0x71900, 0x20);
    fix_hash(&mut reader, 0x71924, 0xbe4);
    fix_hash(&mut reader, 0x73954, 0x16188);
}
