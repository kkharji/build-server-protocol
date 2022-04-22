use lsp_types::{Diagnostic, TextDocumentIdentifier};
use serde::{Deserialize, Serialize};

use super::BuildTargetIdentifier;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishDiagnostics {
    /** The document where the diagnostics are published. */
    text_document: TextDocumentIdentifier,

    /** The build target where the diagnostics origin.
     * It is valid for one text document to belong to multiple
     * build targets, for example sources that are compiled against multiple
     * platforms (JVM, JavaScript). */
    build_target: BuildTargetIdentifier,

    /** The request id that originated this notification. */
    origin_id: Option<String>,

    /** The diagnostics to be published by the client. */
    diagnostics: Vec<Diagnostic>,

    /** Whether the client should clear the previous diagnostics
     * mapped to the same `textDocument` and `buildTarget`. */
    reset: bool,
}
