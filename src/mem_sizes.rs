


#[derive(Clone, Debug)]
pub enum MemorySize {
    Bytes(usize),
    Kilobytes(usize),
    Megabytes(usize),
    Gigabytes(usize),
    Terabytes(usize),
    Petabytes(usize)
}

const MAX_BYTES: usize = usize::MAX;
const MAX_KB: usize = MAX_BYTES / 1024;
const MAX_MB: usize = MAX_KB / 1024;
const MAX_GB: usize = MAX_MB / 1024;
const MAX_TB: usize = MAX_GB / 1024;
const MAX_PB: usize = MAX_TB / 1024;

impl MemorySize {

    pub fn into_bytes(self) -> MemSizeResult {
        Ok(MemorySize::Bytes(match self {
            MemorySize::Bytes(b) => {
                b
            },
            MemorySize::Kilobytes(kb) => {
                if kb > MAX_KB {
                    return Err(OverflowError);
                } else {
                    kb * 1024
                }
            },
            MemorySize::Megabytes(mb) => {
                if mb > MAX_MB {
                    return Err(OverflowError);
                } else {
                    MemorySize::Kilobytes(mb * 1024).into_bytes()?.into()
                }
            },
            MemorySize::Gigabytes(gb) => {
                if gb > MAX_GB {
                    return Err(OverflowError);
                } else {
                    MemorySize::Megabytes(gb * 1024).into_bytes()?.into()
                }
            },
            MemorySize::Terabytes(tb) => {
                if tb > MAX_TB {
                    return Err(OverflowError);
                } else {
                    MemorySize::Gigabytes(tb * 1024).into_bytes()?.into()
                }
            },
            MemorySize::Petabytes(pb) => {
                if pb > MAX_PB {
                    return Err(OverflowError);
                } else {
                    MemorySize::Terabytes(pb * 1024).into_bytes()?.into()
                }
            },
        }))
    }
}

impl From<MemorySize> for usize {
    fn from(m: MemorySize) -> Self {
        match m.into_bytes() {
            Ok(MemorySize::Bytes(size)) => { size },
            Err(e) => { panic!("{:?}", e)},
            _ => { unreachable!() }
        }
    }
}



#[derive(Debug)]
pub struct OverflowError;

pub type MemSizeResult = Result<MemorySize, OverflowError>;





