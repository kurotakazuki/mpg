use mpl::output::Output;
use mpl::parse::Parse;
use mpl::rules::{RightRule, RightRuleKind, Rule, Rules};
use mpl::span::{Span, StartAndLenSpan};
use mpl::symbols::{SliceTerminal, Variable};
use mpl::tree::{AST, CST};
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum WavRiffVariable {
    // RIFF chunk
    Riff,
    FileSize,
    Wave,
    U32,
}

#[derive(Debug)]
enum U16OrU32 {
    // U16(u16),
    U32(u32),
}

impl Variable for WavRiffVariable {}

impl<'input> Output<'input, [u8], WavRiffVariable, StartAndLenSpan<u32, u16>> for U16OrU32 {
    fn output_ast(
        input: &'input [u8],
        cst: CST<Self, WavRiffVariable, StartAndLenSpan<u32, u16>>,
    ) -> AST<Self, WavRiffVariable, StartAndLenSpan<u32, u16>> {
        match cst.node.value {
            WavRiffVariable::U32 => {
                let lo = cst.span.start as usize;
                let hi = cst.span.hi(input) as usize;

                let n = u32::from_le_bytes(input[lo..hi].try_into().unwrap());

                AST::from_cst_and_output(cst, Some(U16OrU32::U32(n)))
            }
            _ => AST::from_cst(cst),
        }
    }
}

/// ```
/// Riff = b"RIFF" FileSize / f
/// FileSize = U32 Wave / f
/// Wave = b"WAVE" () / f
///
/// // U16 = ?? () / f
/// U32 = ???? () / f
/// ```
#[test]
fn wav_riff() {
    let riff_rule: Rule<SliceTerminal<u8>, WavRiffVariable> = Rule::new(
        WavRiffVariable::Riff,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(SliceTerminal::<u8>::Slice(b"RIFF")),
                RightRuleKind::V(WavRiffVariable::FileSize),
            ),
            RightRuleKind::Failure,
        ),
    );
    let file_size_rule: Rule<SliceTerminal<u8>, WavRiffVariable> = Rule::new(
        WavRiffVariable::FileSize,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::V(WavRiffVariable::U32),
                RightRuleKind::V(WavRiffVariable::Wave),
            ),
            RightRuleKind::Failure,
        ),
    );
    let wave_rule: Rule<SliceTerminal<u8>, WavRiffVariable> = Rule::new(
        WavRiffVariable::Wave,
        RightRule::from_right_rule_kind(
            (
                RightRuleKind::T(SliceTerminal::<u8>::Slice(b"WAVE")),
                RightRuleKind::Epsilon,
            ),
            RightRuleKind::Failure,
        ),
    );

    let u32_rule: Rule<SliceTerminal<u8>, WavRiffVariable> = Rule::new(
        WavRiffVariable::U32,
        RightRule::from_right_rule_kind(
            (RightRuleKind::Any(4), RightRuleKind::Epsilon),
            RightRuleKind::Failure,
        ),
    );

    let mut rules = Rules::new();

    rules.insert_rule(riff_rule);
    rules.insert_rule(file_size_rule);
    rules.insert_rule(wave_rule);

    rules.insert_rule(u32_rule);

    let input: &[u8] = &[
        0x52, 0x49, 0x46, 0x46, 0x04, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45,
    ][..];
    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<AST<U16OrU32, WavRiffVariable, StartAndLenSpan<u32, u16>>, ()> =
        input.minimal_parse(&rules, &WavRiffVariable::Riff, all_of_the_span);

    assert!(result.is_ok());

    let input: &[u8] = &[
        0x52, 0x43, 0x46, 0x46, 0x04, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45,
    ][..];
    // all of the span
    let all_of_the_span = StartAndLenSpan::<u32, u16>::from_start_len(0, input.len() as u16);

    let result: Result<AST<U16OrU32, WavRiffVariable, StartAndLenSpan<u32, u16>>, ()> =
        input.minimal_parse(&rules, &WavRiffVariable::Riff, all_of_the_span);

    assert!(result.is_err());
}
