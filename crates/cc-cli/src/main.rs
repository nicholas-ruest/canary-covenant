use cc_api::RolloutApi;
use cc_evaluation::Evaluator;
use cc_orchestrator::{RolloutRequest, Variant};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "canary-covenant")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Evaluate {
        #[arg(long, default_value = "combined")]
        variant: String,
        #[arg(long)]
        harmful: bool,
    },
    VerifyDemo,
}

fn variant(value: &str) -> Result<Variant, String> {
    match value {
        "combined" => Ok(Variant::Combined),
        "no-openfeature" => Ok(Variant::NoOpenFeature),
        "no-ruvector" => Ok(Variant::NoRuVector),
        "no-rvf" => Ok(Variant::NoRvf),
        "no-eprocess" => Ok(Variant::NoEProcess),
        "enterprise-only" => Ok(Variant::EnterpriseOnly),
        "ruvnet-only" => Ok(Variant::RuvnetOnly),
        _ => Err(format!("unknown variant {value}")),
    }
}

#[tokio::main]
async fn main() {
    let result = match Cli::parse().command {
        Command::Evaluate {
            variant: value,
            harmful,
        } => match variant(&value) {
            Ok(selected) => Evaluator::run(selected, harmful)
                .await
                .map(|result| serde_json::to_string_pretty(&result).expect("result serializes")),
            Err(error) => Err(error),
        },
        Command::VerifyDemo => match tempfile::tempdir().map_err(|error| error.to_string()) {
            Ok(work) => RolloutApi::default()
                .evaluate(
                    RolloutRequest {
                        release_id: "verify-demo".into(),
                        version: "2.7.0".into(),
                        digest: "a".repeat(64),
                        observations: Evaluator::fixture(false),
                        variant: Variant::Combined,
                    },
                    work.path(),
                )
                .await
                .map_err(|error| error.to_string())
                .and_then(|report| {
                    if report.verified {
                        Ok(serde_json::to_string_pretty(&report).expect("report serializes"))
                    } else {
                        Err("verification failed".into())
                    }
                }),
            Err(error) => Err(error),
        },
    };

    match result {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_variants_and_rejects_unknown_values() {
        assert_eq!(variant("no-rvf").expect("known"), Variant::NoRvf);
        assert!(variant("bad").is_err());
    }
}
