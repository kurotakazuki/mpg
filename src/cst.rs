use crate::rules::Choice;
use crate::span::{Span, Spanned};
use crate::symbols::{TerminalSymbol, Variable};

type LeafNode<T> = TerminalSymbol<T>;
type InternalNode<T, V, S, L> = Variable<V, Box<Choice<CST<T, V, S, L>>>>;

impl<T, V, S, L> InternalNode<T, V, S, L> {
    pub fn from_first(v: V, l: CST<T, V, S, L>, r: CST<T, V, S, L>) -> Self {
        Variable::new(v, Box::new(Choice::first(l, r)))
    }

    pub fn from_second(v: V, e: CST<T, V, S, L>) -> Self {
        Variable::new(v, Box::new(Choice::second(e)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CSTKind<T, V, S, L> {
    /// Terminal symbol
    LeafNode(LeafNode<T>),
    /// Viriable
    InternalNode(InternalNode<T, V, S, L>),
    // InternalNode { variable: V, choice: Box<Choice<CST<T, V, S, L>>> },
}

pub type CST<T, V, S, L> = Spanned<CSTKind<T, V, S, L>, S, L>;

impl<T, V, S, L> CST<T, V, S, L> {
    pub fn leaf_node(leaf_node: LeafNode<T>, span: Span<S, L>) -> Self {
        Self::new(CSTKind::LeafNode(leaf_node), span)
    }

    pub fn internal_node(internal_node: InternalNode<T, V, S, L>, span: Span<S, L>) -> Self {
        Self::new(CSTKind::InternalNode(internal_node), span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::BytePos;
    use crate::span::{SpanHi, SpanLen};
    use crate::symbols::{Metasymbol, MPGGTerminalType};

    // The following syntax is a lexical syntax for numbers.
    // ```
    // Number = Digit Numeral / f
    // Numeral = Digit Numeral / ()
    // Digit = Zero () / f
    // Zero = "0" () / One
    // One = "1" () / Two
    // // ...
    // Nine = "9" () / f
    // ```
    #[test]
    fn number_cst() {
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum NumberTerminal {
            ZeroFCLHS,
            OneFCLHS,
            TwoFCLHS,
            ThreeFCLHS,
            FourFCLHS,
            FiveFCLHS,
            SixFCLHS,
            SevenFCLHS,
            EightFCLHS,
            NineFCLHS,
        }

        impl NumberTerminal {
            fn into_terminal_type(&self) -> MPGGTerminalType {
                match *self {
                    Self::ZeroFCLHS => "0".into(),
                    Self::OneFCLHS => "1".into(),
                    Self::TwoFCLHS => "2".into(),
                    Self::ThreeFCLHS => "3".into(),
                    Self::FourFCLHS => "4".into(),
                    Self::FiveFCLHS => "5".into(),
                    Self::SixFCLHS => "6".into(),
                    Self::SevenFCLHS => "7".into(),
                    Self::EightFCLHS => "8".into(),
                    Self::NineFCLHS => [96, 1][..].into(),
                }
            }
        }

        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        enum NumberVariable {
            Number,
            Numeral,
            Digit,
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
        }

        // impl<I> SpanHi<u16, I> for BytePos {
        //     fn hi(start: Self, len: u16, _: &I) -> Self {
        //         start + BytePos(len as u32)
        //     }
        // }

        // impl<I> SpanLen<BytePos, I> for u16 {
        //     fn len(lo: BytePos, hi: BytePos, _: &I) -> Self {
        //         u32::from(hi - lo) as u16
        //     }
        // }

        // Input: 10

        // 1
        // One
        let one_fc0: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::Original(NumberTerminal::OneFCLHS),
            Span::from_start_len(BytePos(0), 1),
        );
        let one_fc1: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::M(Metasymbol::Epsilon),
            Span::from_start_len(BytePos(1), 0),
        );
        let one = CST::internal_node(
            InternalNode::from_first(NumberVariable::One, one_fc0, one_fc1),
            Span::from_start_len(BytePos(0), 1),
        );
        // Zero
        let zero = CST::internal_node(
            InternalNode::from_second(NumberVariable::Zero, one),
            Span::from_start_len(BytePos(0), 1),
        );
        // Digit
        let digit_fc1: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::M(Metasymbol::Epsilon),
            Span::from_start_len(BytePos(1), 0),
        );
        let digit_1 = CST::internal_node(
            InternalNode::from_first(NumberVariable::Digit, zero, digit_fc1),
            Span::from_start_len(BytePos(0), 1),
        );

        // 0
        // Zero
        let zero_fc0: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::Original(NumberTerminal::ZeroFCLHS),
            Span::from_start_len(BytePos(1), 1),
        );
        let zero_fc1: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::M(Metasymbol::Epsilon),
            Span::from_start_len(BytePos(2), 0),
        );
        let zero = CST::internal_node(
            InternalNode::from_first(NumberVariable::One, zero_fc0, zero_fc1),
            Span::from_start_len(BytePos(1), 1),
        );
        // Digit
        let digit_fc1: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::M(Metasymbol::Epsilon),
            Span::from_start_len(BytePos(2), 0),
        );
        let digit = CST::internal_node(
            InternalNode::from_first(NumberVariable::Digit, zero, digit_fc1),
            Span::from_start_len(BytePos(1), 1),
        );
        // Numeral
        let numeral_fc1: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::M(Metasymbol::Epsilon),
            Span::from_start_len(BytePos(2), 0),
        );
        let numeral_0 = CST::internal_node(
            InternalNode::from_first(NumberVariable::Digit, digit, numeral_fc1),
            Span::from_start_len(BytePos(1), 1),
        );

        // Number
        let number = CST::internal_node(
            InternalNode::from_first(NumberVariable::Number, digit_1, numeral_0),
            Span::from_start_len(BytePos(0), 2),
        );

        let e: CST<NumberTerminal, NumberVariable, BytePos, u16> = CST::leaf_node(
            LeafNode::M(Metasymbol::Epsilon),
            Span::from_start_len(BytePos(2), 0),
        );

        // assert_eq!(number);
    }
}
