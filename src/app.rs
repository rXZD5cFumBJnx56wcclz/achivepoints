use crate::prelude::*;

pub struct App;

impl App {
    pub fn run(&self) -> impl Future<Output = RResult<()>> {
        async move {
            let aggr_static = AggrStatic::new()?;
            let mut aggr_dyn = AggrDyn::new(&aggr_static, &CLIENTS_MAP)?;
            let mut aggr = Aggr::new(&aggr_static, &mut aggr_dyn);
            let cli = Cli::parse();
            Logic::new(&cli, &mut aggr).interaction().await?;
            Ok(())
        }
    }
}
