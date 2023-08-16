// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn constructor() -> Result<()> {
    return run("ContractMembersList", "constructor");
}

#[test]
fn incomplete_recovery() -> Result<()> {
    return run("ContractMembersList", "incomplete_recovery");
}
