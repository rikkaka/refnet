use anyhow::Result;
use futures::StreamExt;
use refnet_core::doi_to_best_literatures;
use salvo::prelude::*;
use salvo::websocket::Message;

use crate::render_error;
use crate::utils::RenderError;

type Doi = String;

#[handler]
pub async fn best_literatures_from_doi(req: &mut Request, res: &mut Response) -> Result<()> {
    let doi: Doi = render_error!(req.query("doi").ok_or(anyhow::anyhow!("无效的doi")), res);
    let extend_num: usize = render_error!(
        req.query("extend_num")
            .ok_or(anyhow::anyhow!("无效的extend_num")),
        res
    );
    let best_num: usize = render_error!(
        req.query("best_num")
            .ok_or(anyhow::anyhow!("无效的best_num")),
        res
    );
    let alpha: f64 = render_error!(
        req.query("alpha").ok_or(anyhow::anyhow!("无效的alpha")),
        res
    );
    let decay_factor: f64 = render_error!(
        req.query("decay_factor")
            .ok_or(anyhow::anyhow!("无效的decay_factor")),
        res
    );

    let extend_num = extend_num.min(1000);

    let lits = doi_to_best_literatures(doi, extend_num, best_num, alpha, decay_factor).await;
    res.render(Json(lits));

    Ok(())
}

#[handler]
pub async fn literatures_to_review(req: &mut Request, res: &mut Response) -> Result<()> {
    let doi_list: String =
        render_error!(req.query("dois").ok_or(anyhow::anyhow!("无效的dois")), res);
    let dois: Vec<Doi> = doi_list
        .split(',')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect();
    if dois.is_empty() {
        anyhow::bail!("dois为空");
    }
    let mut stream = refnet_core::gen_review(&dois).await;

    WebSocketUpgrade::new()
        .upgrade(req, res, |mut ws| async move {
            stream_to_ws(&mut stream, &mut ws).await;
        })
        .await?;

    async fn stream_to_ws(
        stream: &mut refnet_core::ChatCompletionResponseStream,
        ws: &mut salvo::websocket::WebSocket,
    ) {
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    if let Some(content) = &response.choices[0].delta.content {
                        ws.send(Message::text(content)).await.unwrap();
                    }
                }
                Err(err) => {
                    ws.send(Message::text(&format!("error: {}", err)))
                        .await
                        .unwrap();
                }
            }
        }
    }

    Ok(())
}
