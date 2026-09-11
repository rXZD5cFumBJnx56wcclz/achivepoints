use crate::prelude::*;

pub struct Logic<'a> {
    cli: &'a Cli,
    aggr: &'a mut Aggr<'a>,
}

impl<'a> Logic<'a> {
    pub fn new(cli: &'a Cli, aggr: &'a mut Aggr<'a>) -> Self {
        Self { cli, aggr }
    }
}

impl<'a> Logic<'a> {
    pub fn interaction(&'a mut self) -> impl Future<Output = RResult<()>> {
        async move {
            match &self.cli.sub {
                Subcommands::Run { key } => {
                    LogicRun::new(&self.aggr.dn, key).interaction().await?;
                }
                Subcommands::Cards { command } => {
                    LogicCards::new(self.aggr, command).interaction().await?;
                }
            }
            Ok(())
        }
    }
}
