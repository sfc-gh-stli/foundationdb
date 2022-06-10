// #[allow(non_snake_case)]
#[allow(dead_code, unused_imports)]
#[path = "../../target/flatbuffers/PingRequest_generated.rs"]
mod ping_request;

#[allow(dead_code, unused_imports)]
#[path = "../../target/flatbuffers/Void_generated.rs"]
mod void;

use super::{FlowRequest, FlowResponse};
use crate::flow::file_identifier::{FileIdentifier, IdentifierType, ParsedFileIdentifier};
use crate::flow::frame::Frame;
use crate::flow::uid::UID;
use crate::flow::Result;

const PING_FILE_IDENTIFIER: ParsedFileIdentifier = ParsedFileIdentifier {
    file_identifier: 0x47d2c7,
    inner_wrapper: IdentifierType::None,
    outer_wrapper: IdentifierType::None,
    file_identifier_name: Some("PingRequest"),
};

fn serialize_response(token: UID) -> Result<Frame> {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let void = void::Void::create(&mut builder, &void::VoidArgs {});
    let ensure_table =
        void::EnsureTable::create(&mut builder, &void::EnsureTableArgs { void: Some(void) });
    let fake_root = void::FakeRoot::create(
        &mut builder,
        &void::FakeRootArgs {
            error_or_type: void::ErrorOr::EnsureTable,
            error_or: Some(ensure_table.as_union_value()),
        },
    );
    builder.finish(fake_root, Some("myfi"));
    let (mut payload, offset) = builder.collapse();
    // See also: flow/README.md ### Flatbuffers/ObjectSerializer
    FileIdentifier::new(0x1ead4a)?
        .to_error_or()?
        .rewrite_flatbuf(&mut payload[offset..])?;
    // println!("reply: {:x?}", builder.finished_data());
    Ok(Frame::new(token, payload, offset))
}

pub async fn handle(request: FlowRequest) -> Result<Option<FlowResponse>> {
    request
        .file_identifier
        .ensure_expected(PING_FILE_IDENTIFIER)?;
    let fake_root = ping_request::root_as_fake_root(request.frame.payload())?;
    let reply_promise = fake_root.ping_request().unwrap().reply_promise().unwrap();

    let uid = reply_promise.uid().unwrap();
    let uid = UID {
        uid: [uid.first(), uid.second()],
    };

    let frame = serialize_response(uid)?;
    Ok(Some(FlowResponse { frame }))
}