use crate::deserialization::errors::DeserializationError;

use super::parse_input;

#[test]
fn test_simple_input_parsing() {
    let input = r#"
[
    [
        [
            [14,6,78,90],
            [245,90,0,0,1]
        ],
        [
            [14,6,43,90],
            [9,0,0,0,1]
        ]
    ],
    [
        [
            [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ],
            [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ]
        ],
        [

            [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 14, 0, 0, 0, 45, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ],
            [
                [0, 0, 0, 0, 0, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ]

        ],
        [
            [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 14, 0, 0, 0, 45, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ],
            [
                [0, 0, 0, 0, 0, 98, 0, 0, 0, 156, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ]
        ],
        [
            [
                [0, 0, 0, 0, 0, 98, 0, 0, 0, 156, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [
                    [
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 14, 0, 0, 0, 45, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    ],
                    [
                        [0, 0, 0, 0, 0, 98, 0, 0, 0, 156, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    ]
                ]
            ]
        ],
        [
            [
                [0, 0, 0, 0, 0, 98, 0, 0, 0, 156, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 98, 0, 0, 0, 156, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

            ],
            [
                [0, 0, 0, 1, 0, 98, 0, 0, 0, 156, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 98, 0, 0, 0, 156, 0, 0, 0, 0, 0, 11, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ]

        ]
    ],
    78
]

"#;

    assert!(parse_input(input.to_string()).is_ok());
}

#[test]
fn test_input_parsing_with_storage_key_duplicate() {
    let input = r#"
[
    [
        [
            [14,6,78,90],
            [245,90,0,0,1]
        ],
        [
            [14,6,78,90],
            [9,0,0,0,1]
        ]
    ],
    [
        [],
        [],
        [],
        [],
        []
    ],
    78
]

"#;

    assert!(parse_input(input.to_string())
        .is_err_and(|err| { matches!(err, DeserializationError::KeyDuplicate(_)) }));
}

#[test]
fn test_input_parsing_with_mapping_key_duplicate() {
    let input = r#"
[
    [
        [
            [14,6,78,90],
            [245,90,0,0,1]
        ],
        [
            [0,6,0,90],
            [9,0,0,0,1]
        ]
    ],
    [
        [
            [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ],
            [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 1, 0, 89, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 77, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ]
        ],
        [],
        [],
        [],
        []
    ],
    78
]

"#;

    assert!(parse_input(input.to_string())
        .is_err_and(|err| { matches!(err, DeserializationError::KeyDuplicate(_)) }));
}

#[test]
fn test_input_parsing_valid_tree_size() {
    let input = r#"
[
    [
        [
            [14,6,78,90],
            [245,90,0,0,1]
        ],
        [
            [0,6,0,90],
            [9,0,0,0,1]
        ]
    ],
    [
        [],
        [],
        [],
        [],
        []
    ],
    333
]

"#;

    assert!(parse_input(input.to_string())
        .is_err_and(|err| { matches!(err, DeserializationError::ParsingError(_)) }));
}