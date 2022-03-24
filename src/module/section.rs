pub enum SectionId {
    CustomSectionId = 0,
    TypeSectionId = 1,
    FunctionSectionId = 3,
    ExportSectionId = 7,
    CodeSectionId = 10,
}

impl SectionId {
    pub fn from_usize(n: u8) -> Option<SectionId> {
        match n {
            0 => Some(SectionId::CustomSectionId),
            1 => Some(SectionId::TypeSectionId),
            3 => Some(SectionId::FunctionSectionId),
            7 => Some(SectionId::ExportSectionId),
            10 => Some(SectionId::CodeSectionId),
            _ => todo!(),
        }
    }
}

pub struct TypeSection;
impl TypeSection {
    pub fn validate_header(header: u8) {
        const HEADER: u8 = 0x60;
        if header != HEADER {
            panic!("Invalid TypeSection header {}", header);
        }
    }
}

pub enum ExportDesc {
    Func = 0x00,
    Table = 0x01,
    LinearMemory = 0x02,
    GlobalVariable = 0x03,
}

impl ExportDesc {
    pub fn from_usize(n: u8) -> Option<ExportDesc> {
        match n {
            0 => Some(ExportDesc::Func),
            1 => Some(ExportDesc::Table),
            2 => Some(ExportDesc::LinearMemory),
            3 => Some(ExportDesc::GlobalVariable),
            _ => panic!("Invalid Export Desc {}", n),
        }
    }
}
