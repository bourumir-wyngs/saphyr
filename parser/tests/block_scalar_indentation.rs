use saphyr_parser::{Event, Parser};

#[test]
fn yaml_5llu_block_scalar_wrong_indent_should_fail() {
    // block scalars require consistent indentation.
    let yaml = "block scalar: >\n\n  \n   \n    invalid\n";

    let mut parser = Parser::new_from_str(yaml);

    while let Some(next) = parser.next() {
        match next {
            Ok((Event::DocumentEnd, _)) => {
                assert!(false, "Document end before any error");
            }
            Err(err) => {
                assert_eq!(err.info(), "misplaced bracket",
                           "5LLU: block scalars require consistent indentation"
                );
                break; // fine
            }
            _ => {}
        }
    }
}
