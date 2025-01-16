ast = DeriveInput {
    attrs: [],
    vis: Visibility::Public(
        Pub,
    ),
    ident: Ident {
        ident: "GeneralSettings",
        span: #0 bytes(10021..10036),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Data::Struct {
        struct_token: Struct,
        fields: Fields::Named {
            brace_token: Brace,
            named: [
                Field {
                    attrs: [
                        Attribute {
                            pound_token: Pound,
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::List {
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "default_value",
                                                span: #0 bytes(10045..10058),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                delimiter: MacroDelimiter::Paren(
                                    Paren,
                                ),
                                tokens: TokenStream [
                                    Ident {
                                        ident: "false",
                                        span: #0 bytes(10059..10064),
                                    },
                                ],
                            },
                        },
                    ],
                    vis: Visibility::Public(
                        Pub,
                    ),
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "show_stack_on_push",
                            span: #0 bytes(10075..10093),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "bool",
                                        span: #0 bytes(10095..10099),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
                Field {
                    attrs: [
                        Attribute {
                            pound_token: Pound,
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident {
                                            ident: "no_config",
                                            span: #0 bytes(10107..10116),
                                        },
                                        arguments: PathArguments::None,
                                    },
                                ],
                            },
                        },
                    ],
                    vis: Visibility::Public(
                        Pub,
                    ),
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "show_stack_on_pop",
                            span: #0 bytes(10126..10143),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "bool",
                                        span: #0 bytes(10145..10149),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
                Field {
                    attrs: [
                        Attribute {
                            pound_token: Pound,
                            style: AttrStyle::Outer,
                            bracket_token: Bracket,
                            meta: Meta::NameValue {
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "default_value",
                                                span: #0 bytes(10157..10170),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                eq_token: Eq,
                                value: Expr::Lit {
                                    attrs: [],
                                    lit: Lit::Bool {
                                        value: false,
                                    },
                                },
                            },
                        },
                    ],
                    vis: Visibility::Public(
                        Pub,
                    ),
                    mutability: FieldMutability::None,
                    ident: Some(
                        Ident {
                            ident: "show_books_on_bookmark",
                            span: #0 bytes(10188..10210),
                        },
                    ),
                    colon_token: Some(
                        Colon,
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "bool",
                                        span: #0 bytes(10212..10216),
                                    },
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                },
                Comma,
            ],
        },
        semi_token: None,
    },
}
