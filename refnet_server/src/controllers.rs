use anyhow::Result;
use refnet_core::doi_to_best_literatures;
use salvo::prelude::*;

use crate::render_error;
use crate::utils::RenderError;

type Doi = String;

#[handler]
pub async fn from_doi(req: &mut Request, res: &mut Response) -> Result<()> {
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

// #[handler]
// pub async fn literatures_to_review(req: &mut Request, res: &mut Response) -> Result<()> {
//     todo!()
// }
