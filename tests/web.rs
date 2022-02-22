use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[wasm_bindgen_test]
#[cfg(target_arch = "wasm32")]
fn it_returns_valid_response() {
    use nightrunner_lib::util::test_helpers::mock_json_data;
    use nightrunner_lib::JsMessage;
    use nightrunner_lib::NightRunner;
    let data = mock_json_data();
    let nr = NightRunner::new(&data);

    let result = nr.parse("look");
    assert!(result.is_ok());
    if let Ok(result_data) = result {
        assert_eq!(
            JsValue::into_serde::<JsMessage>(&result_data).unwrap(),
            JsMessage::Look(
                "first room\n\nHere you see: \nan item1\nan item2\nsubject1".to_string()
            )
        )
    } else {
        panic!("result_json is not ok")
    };
}
