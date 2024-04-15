#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::all,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents)]
#![warn(
    clippy::absolute_paths,
    clippy::as_conversions,
    clippy::as_ptr_cast_mut,
    clippy::assertions_on_result_states,
    clippy::branches_sharing_code,
    clippy::clear_with_drain,
    clippy::clone_on_ref_ptr,
    clippy::collection_is_never_read,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::derive_partial_eq_without_eq,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::empty_enum_variants_with_brackets,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::index_refutable_slice,
    clippy::infinite_loop,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_over_hash_type,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::large_stack_frames,
    clippy::let_underscore_untyped,
    clippy::lossy_float_literal,
    clippy::manual_clamp,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::needless_pass_by_ref_mut,
    clippy::needless_raw_strings,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::read_zero_byte_vec,
    clippy::readonly_write_lock,
    clippy::redundant_clone,
    clippy::redundant_type_annotations,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::semicolon_inside_block,
    clippy::shadow_unrelated,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::uninhabited_references,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unnecessary_struct_initialization,
    clippy::unneeded_field_pattern,
    clippy::unused_peekable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    explicit_outlives_requirements,
    future_incompatible,
    let_underscore_drop,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    unit_bindings,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
//! A tool to parse and output as `json` failed tests from Github Actions logs.
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;
use clap::Parser;
use common::{FailedJob, FailedRun};
use futures::future;
use github::{client::GithubClient, workflow::Job};
use tokio::task::JoinHandle;

mod github;
mod log_parser;

/// This command line tool parses failed tests from passed Github
/// Workflows`run_id` and returns failed tests in `output` file.
#[derive(Debug, Parser)]
struct Cli {
    /// The run id of the finished workflow.
    #[clap(long, short)]
    run_id: u64,

    /// The Github token to make requests. Needs `repo` scope.
    #[clap(long, short = 't')]
    github_token: String,

    /// The owner and repository name in the format `owner/repo`
    #[clap(long)]
    owner_repo: String,

    /// The output`.json` file to save the failed tests. Also this is the place
    /// where previous runs are searched for.
    #[clap(long, short, default_value = "runs.json")]
    output: String,

    /// The number of runs to keep in the output file. New runs will be placed
    /// first and all runs will be truncated to this number.
    #[clap(long, short, default_value_t = 20)]
    cutoff: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let run_id = cli.run_id;

    let client = GithubClient::new(cli.github_token, cli.owner_repo);

    let failed_jobs = client.get_failed_workflow_jobs(cli.run_id).await?;

    let loged_failures = parse_logs(failed_jobs, &client).await;

    save_to_file(
        FailedRun {
            run_id,
            jobs: loged_failures
                .into_iter()
                .filter(|j| !j.failures.is_empty())
                .collect(),
        },
        &cli.output,
        cli.cutoff,
    )?;

    Ok(())
}

/// Merges previous failed runs with the new one and saves them to the output.
/// All runs after the cutoff will be truncated.
fn save_to_file(
    loged_failure: FailedRun,
    output: &str,
    cutoff: u32,
) -> Result<()> {
    let file = File::open(output)?;
    let prev_failures: Vec<FailedRun> = serde_json::from_reader(file)?;

    let mut failed_runs = vec![loged_failure];
    failed_runs.extend(prev_failures);

    failed_runs.truncate(cutoff.try_into().unwrap_or(usize::MAX));

    let write_file = File::create(output)?;
    let mut writer = BufWriter::new(write_file);
    serde_json::to_writer(&mut writer, &failed_runs)?;
    writer.flush()?;

    Ok(())
}

/// Parses logs of failed jobs and returns a list of failed tests. Uses `client`
/// to download logs of each job.
async fn parse_logs(jobs: Vec<Job>, client: &GithubClient) -> Vec<FailedJob> {
    let tasks =
        jobs.into_iter()
            .map(|job| {
                let client = client.clone();
                tokio::spawn(async move {
                    client.download_job_logs(&job).await.ok().map(|l| {
                        FailedJob {
                        job: job.name,
                        failures:
                            log_parser::FailedTestsParser::parse_failed_tests(
                                &l,
                                client.owner_repo.as_str(),
                            ),
                    }
                    })
                })
            })
            .collect::<Vec<JoinHandle<Option<FailedJob>>>>();

    future::try_join_all(tasks)
        .await
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .collect()
}
