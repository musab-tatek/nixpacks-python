use nixpacks::{
    create_docker_image, generate_build_plan, get_plan_providers,
    nixpacks::builder::docker::DockerBuilderOptions,
    nixpacks::nix::pkg::Pkg,
    nixpacks::plan::phase::{Phase, StartPhase},
    nixpacks::plan::{generator::GeneratePlanOptions, BuildPlan},
};
use pyo3::prelude::*;

#[pyfunction]
fn detect(
    path: &str,
    environment_variables: Option<Vec<&str>>,
    config_file: Option<String>,
) -> PyResult<String> {
    let plan_options = GeneratePlanOptions {
        plan: None,
        config_file: config_file,
    };

    let providers = get_plan_providers(
        &path,
        environment_variables.unwrap_or_default(),
        &plan_options,
    );
    match providers {
        Ok(providers) => Ok(providers.join(", ")),
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyEnvironmentError, _>(
            format!("Error generating build plan: {}", err),
        )),
    }
}

#[pyfunction]
fn plan(
    path: &str,
    environment_variables: Option<Vec<&str>>,
    json_plan: Option<String>,
    install_cmds: Option<Vec<String>>,
    build_cmds: Option<Vec<String>>,
    start_cmd: Option<String>,
    apt_pkgs: Option<Vec<String>>,
    nix_pkgs: Option<Vec<String>>,
    nix_libs: Option<Vec<String>>,
) -> PyResult<String> {
    let mut build_plan = BuildPlan::default();

    if apt_pkgs.is_some() || nix_pkgs.is_some() || nix_libs.is_some() {
        let nix_pkgs = nix_pkgs
            .map(|pkgs| pkgs.into_iter().map(|pkg| Pkg::new(&pkg)).collect())
            .map(|pkgs| Some(pkgs))
            .unwrap_or_else(|| None);

        let mut setup = Phase::setup(nix_pkgs);

        setup.apt_pkgs = apt_pkgs;
        setup.nix_libs = nix_libs;
        build_plan.add_phase(setup);
    }

    if install_cmds.is_some() {
        let mut install = Phase::install(None);
        install.cmds = install_cmds;
        build_plan.add_phase(install);
    }

    if build_cmds.is_some() {
        let mut build = Phase::build(None);
        build.cmds = build_cmds;
        build_plan.add_phase(build);
    }

    if start_cmd.is_some() {
        let start = StartPhase::new(start_cmd.unwrap());
        build_plan.set_start_phase(start);
    }

    let json_plan = match json_plan.map(BuildPlan::from_json).transpose() {
        Ok(plan) => plan,
        Err(err) => {
            return Err(PyErr::new::<pyo3::exceptions::PyEnvironmentError, _>(
                format!("Error generating build plan: {}", err),
            ))
        }
    };

    let build_plan = if let Some(json_plan) = json_plan {
        BuildPlan::merge_plans(&[json_plan, build_plan])
    } else {
        build_plan
    };

    let plan_options = GeneratePlanOptions {
        plan: Some(build_plan),
        config_file: None,
    };
    let plan_result = generate_build_plan(
        path,
        environment_variables.unwrap_or_default(),
        &plan_options,
    );

    match plan_result {
        Ok(plan) => {
            let json_string = match plan.to_json() {
                Ok(json) => json,
                Err(err) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Error generating build plan: {}",
                        err
                    )))
                }
            };
            Ok(json_string)
        }
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Error generating build plan: {}",
            err
        ))),
    }
}

#[pyfunction]
fn build(
    path: &str,
    name: &str,
    out_dir: Option<String>,
    print_dockerfile: Option<bool>,
    tags: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    quiet: Option<bool>,
    cache_key: Option<String>,
    no_cache: Option<bool>,
    inline_cache: Option<bool>,
    cache_from: Option<String>,
    platform: Option<Vec<String>>,
    current_dir: Option<bool>,
    no_error_without_start: Option<bool>,
    incremental_cache_image: Option<String>,
    cpu_quota: Option<String>,
    memory: Option<String>,
    verbose: Option<bool>,
    docker_host: Option<String>,
    docker_tls_verify: Option<String>,
    environment_variables: Option<Vec<&str>>,
    config_file: Option<String>,
) {
    let plan_options = GeneratePlanOptions {
        plan: None,
        config_file: config_file,
    };

    let build_options = &DockerBuilderOptions {
        name: Some(name.to_string()),
        out_dir, // Some("/output".to_string()),
        print_dockerfile: print_dockerfile.unwrap_or(false),
        tags: tags.unwrap_or_default(), // vec!["latest".to_string(), "v1.0".to_string()],
        labels: labels.unwrap_or_default(), // vec!["author=John Doe".to_string(), "version=1.0".to_string()],
        quiet: quiet.unwrap_or_else(|| verbose.unwrap_or(false)),
        cache_key, // Some("my_cache_key".to_string()),
        no_cache: no_cache.unwrap_or(false),
        inline_cache: inline_cache.unwrap_or(true),
        cache_from, // Some("previous_image:latest".to_string()),
        platform: platform.unwrap_or_default(), // vec!["linux/amd64".to_string()],
        current_dir: current_dir.unwrap_or(true),
        no_error_without_start: no_error_without_start.unwrap_or(false),
        incremental_cache_image, // Some("incremental_cache:latest".to_string()),
        cpu_quota,               // Some("2".to_string()),
        memory,                  // Some("4G".to_string()),
        verbose: verbose.unwrap_or(false),
        docker_host,       // Some("tcp://localhost:2375".to_string()),
        docker_tls_verify, // Some("/path/to/cert.pem".to_string()),
    };

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let _ = create_docker_image(
            &path,
            environment_variables.unwrap_or_default(),
            &plan_options,
            &build_options,
        )
        .await;
    });
}

#[pymodule]
fn nixpacks_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect, m)?)?;
    m.add_function(wrap_pyfunction!(plan, m)?)?;
    m.add_function(wrap_pyfunction!(build, m)?)?;
    Ok(())
}
