use uncollate::Uncollate;

#[derive(Uncollate)]
pub struct Os {
    opt: Option<String>
}

#[test]
fn uncollate_option_string() {
    let opts = vec![
        Os { opt: Some("apple".into())},
        Os { opt: None },
        Os { opt: Some("banana".into())}
    ];

    let uncol = opts.uncollate();

    assert_eq!(uncol.opt[0], Some("apple".into()));
    assert_eq!(uncol.opt[1], None);
    assert_eq!(uncol.opt[2], Some("banana".into()));
}