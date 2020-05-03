// To run from the root directory:
//
//     cargo run -p sync-test -- --oauth-retries 5
//
// (You can safely ignore the noisy 500 for
// `https://stable.dev.lcip.org/auth/v1/account/destroy` at the end).

use failure::Error;
use interrupt::NeverInterrupts;
use log::*;
use serde_derive::*;
use sync15::{
    telemetry, CollectionRequest, IncomingChangeset, MemoryCachedState, OutgoingChangeset, Payload,
    ServerTimestamp, Store, StoreSyncAssociation,
};
use sync_guid::Guid;
use std::cell::RefCell;
use std::borrow::BorrowMut;

use crate::auth::TestClient;
use crate::testing::TestGroup;

// A test record. It has to derive `Serialize` and `Deserialize` (which we import
// in scope when we do `use serde_derive::*`), so that the `sync15` crate can
// serialize them to JSON, and parse them from JSON. Deriving `Debug` lets us
// print it with `{:?}` below.
#[derive(Debug, Deserialize, Serialize)]
struct TestRecord {
    // This field is required for all Sync records, but can be set to whatever
    // value we want. In the test, we just generate a random GUID.
    id: Guid,
    // And a test field for our record.
    test1: String,
}

// Actual tests.

fn sync_first_client(c0: &mut TestClient) -> TestStore {
    let (init, key, _device_id) = c0
        .data_for_sync()
        .expect("Should have data for syncing first client");

    let store = TestStore { test_record: "".to_string() };
    let mut persisted_global_state = None;
    let mut mem_cached_state = MemoryCachedState::default();
    let result = sync15::sync_multiple(
        &[&store],
        &mut persisted_global_state,
        &mut mem_cached_state,
        &init,
        &key,
        &NeverInterrupts,
        None,
    );
    println!("Finished syncing first client: {:?}", result);

    return store;
}

fn sync_second_client(c1: &mut TestClient) -> TestStore {
    let (init, key, _device_id) = c1
        .data_for_sync()
        .expect("Should have data for syncing second client");

    let store = TestStore { test_record: "".to_string() };
    let mut persisted_global_state = None;
    let mut mem_cached_state = MemoryCachedState::default();
    let result = sync15::sync_multiple(
        &[&store],
        &mut persisted_global_state,
        &mut mem_cached_state,
        &init,
        &key,
        &NeverInterrupts,
        None,
    );
    println!("Finished syncing second client: {:?}", result);

    return store;
}

// Call tests.

// (It works when the email account is successfully created)
fn test_sync_multiple(c0: &mut TestClient, c1: &mut TestClient) {
    sync_first_client(c0);
    sync_second_client(c1);

    let s0 = TestStore {
        test_record: "<333".to_string()
    };
    // HERE
    c0.sync_multiple_engine.

    info!("\n\n\n ASSERT:");
    assert_eq!(store1.message, store2.message);
    info!("\n\n\n")
}

// Boilerplate...
pub fn get_test_group() -> TestGroup {
    TestGroup::new("sync15",
                   vec![("test_sync_multiple", test_sync_multiple)])
}
