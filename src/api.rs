use leptos::{Scope, ServerFnError, server};

use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(
    cx: Scope,
    prompt: Conversation
) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use laptos_actix::extract;
    use actix_web::{
        web::Data,
        dev::ConnectionInfo,
    };

    
}