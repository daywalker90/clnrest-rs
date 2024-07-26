use cln_plugin::Plugin;
use cln_rpc::{model::responses::CheckruneResponse, ClnRpc, RpcError};
use serde_json::json;

use crate::{handlers::AppError, PluginState};

pub async fn verify_rune(
    plugin: Plugin<PluginState>,
    rune_header: Option<String>,
    rpc_method: &str,
    rpc_params: &serde_json::Value,
) -> Result<(), AppError> {
    // Implement this function
    if let Some(rune) = rune_header {
        match call_rpc(
            plugin,
            "checkrune",
            json!( {"rune": rune,
            "method": rpc_method,
            "params": rpc_params}),
        )
        .await
        {
            Ok(o) => {
                let check_resp: CheckruneResponse = serde_json::from_value(o).unwrap();
                if check_resp.valid {
                    Ok(())
                } else {
                    let err = RpcError {
                        code: Some(1502),
                        message: "Rune is not valid".to_string(),
                        data: None,
                    };
                    log::info!("{}", err);
                    Err(AppError::Unauthorized(err))
                }
            }
            Err(e) => {
                log::info!("{}", e);
                Err(AppError::Unauthorized(e))
            }
        }
    } else {
        let err = RpcError {
            code: Some(1501),
            data: None,
            message: "Not authorized: Missing rune".to_string(),
        };
        log::info!("{}", err);
        Err(AppError::Forbidden(err))
    }
}

// Implement your process_help_response, call_rpc_method, and verify_rune functions
pub async fn call_rpc(
    plugin: Plugin<PluginState>,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, RpcError> {
    // Implement this function
    let rpc_path = plugin.configuration().rpc_file;
    let mut rpc = ClnRpc::new(rpc_path).await.map_err(|e| RpcError {
        code: None,
        data: None,
        message: e.to_string(),
    })?;
    log::debug!("call_rpc: method:{} params:{:?}", method, params);
    let response: serde_json::Value = rpc.call_raw(method, &params).await?;
    log::debug!("call_rpc: response:{}", response);
    Ok(response)
}

pub fn filter_json(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Array(arr) => {
            for v in arr {
                filter_json(v);
            }
        }
        serde_json::Value::Object(obj) => {
            obj.retain(|k, v| !is_unwanted(k, v));
            for v in obj.values_mut() {
                filter_json(v);
            }
        }
        _ => (),
    }
}

fn is_unwanted(key: &String, value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => {
            log::debug!("is_unwanted: key:{} value:{} (null)", key, value);
            true
        }
        _ => false,
    }
}
