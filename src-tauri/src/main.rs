#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use quickentity_rs::{
	convert_qn_property_value_to_rt, convert_rt_property_value_to_qn, qn_structs::Property,
	rt_structs::SEntityTemplatePropertyValue,
};
use tauri::generate_handler;

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_fs_watch::init())
		.plugin(tauri_plugin_websocket::init())
		.invoke_handler(generate_handler![
			convert_property_value_to_qn,
			convert_property_value_to_rt
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
fn convert_property_value_to_qn(
	value: SEntityTemplatePropertyValue,
) -> Result<serde_json::Value, String> {
	convert_rt_property_value_to_qn(
		&value,
		&Default::default(),
		&Default::default(),
		&Default::default(),
		false,
	)
	.map_err(|x| x.to_string())
}

#[tauri::command]
fn convert_property_value_to_rt(value: Property) -> Result<serde_json::Value, String> {
	convert_qn_property_value_to_rt(
		&value,
		&Default::default(),
		&Default::default(),
		&Default::default(),
		&Default::default(),
	)
	.map_err(|x| x.to_string())
}
