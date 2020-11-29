#![allow(unused)]

mod text_document {
    mod did_open {
        fn handler(_corpus: &str, path: &str) {
            use ddlog_lsp_parsers::core::language::Language;
            use ddlog_lsp_testing as testing;
            use futures::stream::StreamExt;
            use serde_json::Value;
            use std::convert::TryFrom;
            use tower_lsp::lsp_types::*;

            async fn handler(path: &str) -> anyhow::Result<()> {
                let uri = Url::from_file_path(path).unwrap();
                let text = std::fs::read_to_string(path).unwrap();
                let language = Language::try_from(std::path::Path::new(path))?;
                let language_id = language.id();

                let (ref mut service, ref mut messages) = testing::service::spawn()?;

                // send "initialize" request
                testing::assert_status!(service, Ok(()));
                let request = &testing::lsp::initialize::request();
                let response = Some(testing::lsp::initialize::response());
                testing::assert_exchange!(service, request, Ok(response));

                // send "initialized" notification
                testing::assert_status!(service, Ok(()));
                let notification = &testing::lsp::initialized::notification();
                let status = None::<Value>;
                testing::assert_exchange!(service, notification, Ok(status));
                // ignore the "window/logMessage" notification: "WebAssembly language server initialized!"
                messages.next().await.unwrap();

                // send "textDocument/didOpen" notification for `uri`
                testing::assert_status!(service, Ok(()));
                let notification = &testing::lsp::text_document::did_open::notification(&uri, language_id, 1, text);
                let status = None::<Value>;
                testing::assert_exchange!(service, notification, Ok(status));

                // receive "textDocument/publishDiagnostics" notification for `uri`
                let message = messages.next().await.unwrap();
                let actual = serde_json::to_value(&message)?;
                let expected = testing::lsp::text_document::publish_diagnostics::notification(&uri, &[]);
                assert_eq!(actual, expected);

                Ok(())
            }
            let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
            runtime.block_on(handler(path)).unwrap();
        }

        mod differential_datalog {
            use ddlog_lsp_macros::corpus_tests;

            corpus_tests! {
                corpus: antrea,
                include: "vendor/differential-datalog/test/antrea/**/*.dl",
                handler: crate::lsp::text_document::did_open::handler,
            }

            corpus_tests! {
                corpus: datalog_tests,
                include: "vendor/differential-datalog/test/datalog_tests/**/*.dl",
                handler: crate::lsp::text_document::did_open::handler,
            }

            corpus_tests! {
                corpus: types_tests,
                include: "vendor/differential-datalog/test/types_tests/**/*.dl",
                handler: crate::lsp::text_document::did_open::handler,
            }
        }
    }
}
