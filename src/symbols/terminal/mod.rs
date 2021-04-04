use crate::cst::CST;

pub mod metasymbol;
pub mod str_terminal;
pub mod terminal_symbol;
pub mod u8slice_terminal;

pub trait Terminal<'input, I, OutputT, V, S, P>
where
    I: ?Sized,
{
    fn eval(
        &'input self,
        input: &'input I,
        pos: P,
        all_of_the_span: &S,
    ) -> Result<CST<OutputT, V, S>, ()>;
}
