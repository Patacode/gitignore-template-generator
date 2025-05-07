use criterion::{Criterion, criterion_group, criterion_main};
use gitignore_template_generator::helper::load_expectation_file_as_string;
use mockito::Server;
use test_bin::get_test_bin;

fn benchmark_function(c: &mut Criterion) {
    let mut mock_server = Server::new();
    let mock_server_base_url = mock_server.url();
    let template_generator_service_uri = "/custom/rust";

    mock_server
        .mock("GET", template_generator_service_uri)
        .with_status(200)
        .with_body(load_expectation_file_as_string("rust_template"))
        .create();

    c.bench_function(env!("CARGO_PKG_NAME"), |b| {
        b.iter(|| {
            let mut cli_tool = get_test_bin(env!("CARGO_PKG_NAME"));

            cli_tool
                .arg("rust")
                .args(["--server-url", &mock_server_base_url])
                .args(["--generator-uri", "/custom"]);

            cli_tool.output().unwrap();
        })
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
