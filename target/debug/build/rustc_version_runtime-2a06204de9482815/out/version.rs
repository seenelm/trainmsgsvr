
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 74,
                        patch: 1,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "aarch64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.74.1 (a28077b28 2023-12-04)".to_owned(),
                    commit_hash: Some("a28077b28a02b92985b3a3faecf92813155f1ea1".to_owned()),
                    commit_date: Some("2023-12-04".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            