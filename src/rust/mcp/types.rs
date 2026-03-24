use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XuRequest {
    #[schemars(description = "要显示给用户的消息")]
    pub message: String,
    #[schemars(description = "预定义的选项列表（可选）")]
    #[serde(default)]
    pub predefined_options: Vec<String>,
    #[schemars(description = "消息是否为Markdown格式，默认为true")]
    #[serde(default = "default_is_markdown")]
    pub is_markdown: bool,
}

fn default_is_markdown() -> bool {
    true
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JiyiRequest {
    #[schemars(description = "操作类型：记忆(添加记忆), 回忆(获取项目信息)")]
    pub action: String,
    #[schemars(description = "项目路径（必需）")]
    pub project_path: String,
    #[schemars(description = "记忆内容（记忆操作时必需）")]
    #[serde(default)]
    pub content: String,
    #[schemars(
        description = "记忆分类：rule(规范规则), preference(用户偏好), pattern(最佳实践), context(项目上下文)"
    )]
    #[serde(default = "default_category")]
    pub category: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AcemcpRequest {
    #[schemars(description = "项目根目录的绝对路径，使用正斜杠(/)作为分隔符")]
    pub project_root_path: String,
    #[schemars(description = "用于查找相关代码上下文的自然语言搜索查询")]
    pub query: String,
}

fn default_category() -> String {
    "context".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PopupRequest {
    pub id: String,
    #[serde(default)]
    pub session_id: String,
    pub message: String,
    pub predefined_options: Option<Vec<String>>,
    pub is_markdown: bool,
}

/// 新的结构化响应数据格式
#[derive(Debug, Deserialize)]
pub struct McpResponse {
    pub user_input: Option<String>,
    pub selected_options: Vec<String>,
    pub images: Vec<ImageAttachment>,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAttachment {
    pub data: String,
    pub media_type: String,
    pub filename: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetadata {
    pub timestamp: Option<String>,
    pub request_id: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    pub source: Option<String>,
}

/// 旧格式兼容性支持
#[derive(Debug, Deserialize)]
pub struct McpResponseContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub source: Option<ImageSource>,
}

#[derive(Debug, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

/// 统一的响应构建函数
///
/// 用于生成标准的JSON响应格式，确保无GUI和有GUI模式输出一致
fn resolve_session_id(session_id: Option<String>, request_id: &Option<String>) -> Option<String> {
    match session_id {
        Some(session_id) if !session_id.trim().is_empty() => Some(session_id),
        _ => request_id.clone(),
    }
}

pub fn popup_session_id(request: &PopupRequest) -> Option<String> {
    resolve_session_id(Some(request.session_id.clone()), &Some(request.id.clone()))
}

pub fn build_mcp_response(
    user_input: Option<String>,
    selected_options: Vec<String>,
    images: Vec<ImageAttachment>,
    request_id: Option<String>,
    session_id: Option<String>,
    source: &str,
) -> serde_json::Value {
    let session_id = resolve_session_id(session_id, &request_id);
    serde_json::json!({
        "user_input": user_input,
        "selected_options": selected_options,
        "images": images,
        "metadata": {
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "request_id": request_id,
            "session_id": session_id,
            "source": source
        }
    })
}

/// 构建发送操作的响应
pub fn build_send_response(
    user_input: Option<String>,
    selected_options: Vec<String>,
    images: Vec<ImageAttachment>,
    request_id: Option<String>,
    session_id: Option<String>,
    source: &str,
) -> String {
    let response = build_mcp_response(
        user_input,
        selected_options,
        images,
        request_id,
        session_id,
        source,
    );
    response.to_string()
}

/// 构建继续操作的响应
pub fn build_continue_response(
    request_id: Option<String>,
    session_id: Option<String>,
    source: &str,
) -> String {
    // 动态获取继续提示词
    let continue_prompt = if let Ok(config) = crate::config::load_standalone_config() {
        config.reply_config.continue_prompt
    } else {
        "请按照最佳实践继续".to_string()
    };

    let response = build_mcp_response(
        Some(continue_prompt),
        vec![],
        vec![],
        request_id,
        session_id,
        source,
    );
    response.to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        build_continue_response, build_mcp_response, build_send_response, popup_session_id,
        PopupRequest,
    };

    #[test]
    fn build_mcp_response_preserves_explicit_session_id() {
        let response = build_mcp_response(
            Some("继续处理".to_string()),
            vec!["保留".to_string()],
            vec![],
            Some("request-123".to_string()),
            Some("session-abc".to_string()),
            "popup",
        );

        assert_eq!(response["metadata"]["request_id"], "request-123");
        assert_eq!(response["metadata"]["session_id"], "session-abc");
    }

    #[test]
    fn build_mcp_response_falls_back_to_request_id_when_session_id_missing() {
        let response = build_mcp_response(
            Some("继续处理".to_string()),
            vec![],
            vec![],
            Some("request-123".to_string()),
            None,
            "popup",
        );

        assert_eq!(response["metadata"]["request_id"], "request-123");
        assert_eq!(response["metadata"]["session_id"], "request-123");
    }

    #[test]
    fn build_send_response_serializes_explicit_session_id() {
        let response = build_send_response(
            Some("继续处理".to_string()),
            vec!["保留".to_string()],
            vec![],
            Some("request-123".to_string()),
            Some("session-abc".to_string()),
            "popup",
        );
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();

        assert_eq!(response["metadata"]["request_id"], "request-123");
        assert_eq!(response["metadata"]["session_id"], "session-abc");
    }

    #[test]
    fn build_continue_response_serializes_explicit_session_id() {
        let response = build_continue_response(
            Some("request-123".to_string()),
            Some("session-abc".to_string()),
            "popup_continue",
        );
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();

        assert_eq!(response["metadata"]["request_id"], "request-123");
        assert_eq!(response["metadata"]["session_id"], "session-abc");
    }

    #[test]
    fn popup_session_id_falls_back_to_request_id_for_legacy_requests() {
        let request = PopupRequest {
            id: "request-123".to_string(),
            session_id: String::new(),
            message: "继续处理".to_string(),
            predefined_options: None,
            is_markdown: true,
        };

        assert_eq!(popup_session_id(&request).as_deref(), Some("request-123"));
    }
}
