use wasm_bindgen::prelude::*;
use js_sys::Function;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities"])]
    pub async fn create(details: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities"])]
    pub async fn get(cookieStoreId: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities"])]
    pub async fn query(details: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities"])]
    pub async fn update(cookieStoreId: &str, details: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities"])]
    pub async fn remove(cookieStoreId: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onCreated"])]
    pub fn add_onCreated_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onCreated"])]
    pub fn remove_onCreated_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onCreated"])]
    pub fn has_onCreated_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onRemoved"])]
    pub fn add_onRemoved_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onRemoved"])]
    pub fn remove_onRemoved_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onRemoved"])]
    pub fn has_onRemoved_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onUpdated"])]
    pub fn add_onUpdated_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onUpdated"])]
    pub fn remove_onUpdated_listener(listener: &Function) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["browser", "contextualIdentities", "onUpdated"])]
    pub fn has_onUpdated_listener(listener: &Function) -> Result<JsValue, JsValue>;
}
