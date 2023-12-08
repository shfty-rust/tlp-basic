// Type level boolean
pub struct True;
pub struct False;

pub trait Boolean {
    const VALUE: bool;
}

impl Boolean for True {
    const VALUE: bool = true;
}

impl Boolean for False {
    const VALUE: bool = false;
}

pub trait NotOp {
    type Not;
}

impl NotOp for True {
    type Not = False;
}

impl NotOp for False {
    type Not = True;
}

pub type Not<B> = <B as NotOp>::Not;

