#[derive(Debug)]
pub enum Instruction {
    Hlf { reg: String },
    Tpl { reg: String },
    Inc { reg: String },
    Jmp { offset: isize },
    Jie { reg: String, offset: isize },
    Jio { reg: String, offset: isize },
}
