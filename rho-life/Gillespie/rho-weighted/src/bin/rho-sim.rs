//! `rho-sim` — run a study and emit graphs.
//!
//! The simulator's product is graphs, so the binary's job is to produce them:
//! a DOT rendering of the labelled transition system, the generator as CSV, and
//! the study result as JSON with its provenance.

use std::io::Write;

use rho_weighted::examples::{birth_death, rnn, synapse_class, two_state, NetSpec};
use rho_weighted::explore::exhaustive_graph;
use rho_weighted::matching::SimpleMatcher;
use rho_weighted::space::Space;
use rho_weighted::study::{sweep_1d, Study};
use rho_weighted::theory::Configuration;

fn usage() -> ! {
    eprintln!(
        "usage: rho-sim <model> [outdir]\n\
         \n\
         models:\n\
         \x20 two-state      a two-state chain; closed-form stationary\n\
         \x20 birth-death    occupancy is Poisson because multiplicity is counted\n\
         \x20 rnn            the recurrent pair, swept over synaptic efficacy\n"
    );
    std::process::exit(2)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
    }
    let outdir = args.get(2).cloned().unwrap_or_else(|| "out".into());
    std::fs::create_dir_all(&outdir)?;

    let (name, result, dot, csv) = match args[1].as_str() {
        "two-state" => {
            let (theory, term) = two_state(2.0, 0.5);
            let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
            let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 1000);
            let study = Study::new(&theory, &SimpleMatcher, cfg)
                .with_ensemble(8, 200.0)
                .run();
            (
                "two-state",
                study.to_json(),
                ex.graph.to_dot("two-state"),
                ex.generator.to_csv(&ex.graph.node_labels),
            )
        }
        "birth-death" => {
            let (theory, term) = birth_death(3.0, 1.0, 10);
            let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
            let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 2000);
            let study = Study::new(&theory, &SimpleMatcher, cfg)
                .with_state_budget(2000)
                .run();
            (
                "birth-death",
                study.to_json(),
                ex.graph.to_dot("birth-death"),
                ex.generator.to_csv(&ex.graph.node_labels),
            )
        }
        "rnn" => {
            let spec = NetSpec {
                post: vec![vec![1], vec![0]],
                theta: vec![1, 1],
                initial: vec![((0, 1), 2)],
            };
            let (theory, term, syns) = rnn(&spec, |_, _| 1.0, None, 8);
            let cfg = Configuration::new(Space::install(&term), theory.initial_weights());
            let ex = exhaustive_graph(&theory, &cfg, &SimpleMatcher, 1000);
            let k = synapse_class(&syns, 0, 1).expect("synapse (0,1)");
            let study = Study::new(&theory, &SimpleMatcher, cfg)
                .with_grid(sweep_1d(k, "w01", &[0.25, 0.5, 1.0, 2.0, 4.0]))
                .with_state_budget(1000)
                .run();
            (
                "rnn",
                study.to_json(),
                ex.graph.to_dot("rnn"),
                ex.generator.to_csv(&ex.graph.node_labels),
            )
        }
        _ => usage(),
    };

    write(&outdir, &format!("{name}.json"), &result)?;
    write(&outdir, &format!("{name}.dot"), &dot)?;
    write(&outdir, &format!("{name}-generator.csv"), &csv)?;
    println!("wrote {outdir}/{name}.{{json,dot}} and {outdir}/{name}-generator.csv");
    Ok(())
}

fn write(dir: &str, name: &str, content: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(format!("{dir}/{name}"))?;
    f.write_all(content.as_bytes())
}
