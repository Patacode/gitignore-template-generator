use criterion::{Criterion, criterion_group, criterion_main};
use gitignore_template_generator::helper::load_expectation_file_as_string;
use mockito::{Server, ServerGuard};
use test_bin::get_test_bin;

fn setup_mock_server() -> ServerGuard {
    let mut mock_server = Server::new();
    let template_generator_service_uri = "/custom/rust";
    let template_lister_service_uri = "/custom/list";

    mock_server
        .mock("GET", template_generator_service_uri)
        .with_status(200)
        .with_body(load_expectation_file_as_string("rust_template"))
        .create();
    mock_server
        .mock("GET", template_lister_service_uri)
        .with_status(200)
        .with_body(load_expectation_file_as_string("template_list"))
        .create();

    mock_server
}

fn generate_template(server_base_url: &str, with_robust_check: bool) {
    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

    cli_tool
        .arg("rust")
        .args(["--server-url", server_base_url])
        .args(["--generator-uri", "/custom"]);

    if with_robust_check {
        cli_tool.arg("--check");
    }

    cli_tool.output().unwrap();
}

fn list_templates(server_base_url: &str) {
    let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

    cli_tool
        .arg("--list")
        .args(["--server-url", server_base_url])
        .args(["--lister-uri", "/custom/list"]);

    cli_tool.output().unwrap();
}

fn benchmark_template_generation(c: &mut Criterion) {
    let mock_server = setup_mock_server();
    let mock_server_base_url = mock_server.url();

    let mut template_generation_group = c.benchmark_group("template/generation");

    template_generation_group.bench_function("Template generation without robust check", |b| {
        b.iter(|| generate_template(&mock_server_base_url, false))
    });
    template_generation_group.bench_function("Template generation with robust check", |b| {
        b.iter(|| generate_template(&mock_server_base_url, true))
    });

    template_generation_group.finish();
}

fn benchmark_template_listing(c: &mut Criterion) {
    let mock_server = setup_mock_server();
    let mock_server_base_url = mock_server.url();

    let mut template_listing_group = c.benchmark_group("template/listing");

    template_listing_group.bench_function("Template listing", |b| {
        b.iter(|| list_templates(&mock_server_base_url))
    });

    template_listing_group.finish();
}

criterion_group!(
    benches,
    benchmark_template_generation,
    benchmark_template_listing
);
criterion_main!(benches);
