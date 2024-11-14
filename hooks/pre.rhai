// Values specified on the CLI with `--define` will always be strings
fn check_bool(variable_name) {
    if !variable::is_set(variable_name) {
        return false;
    }

    let value = variable::get(variable_name);
    if type_of(value) == "string" {
        return value == "true";
    } else {
        return value;
    }
}

if !variable::is_set("project-description") {
    let project_description = variable::prompt("A short description");
    variable::set("project-description", project_description);
}

if !variable::is_set("project-homepage") {
    let project_homepage = variable::prompt("Your project homepage");
    variable::set("project-homepage", project_homepage);
}

if !variable::is_set("flakes") {
    let flakes = variable::prompt("Do you want to use Nix & Nix Flakes?", false);
    variable::set("flakes", flakes);
}

if !check_bool("flakes") {
    file::delete("./flake.nix");
    file::delete("./flake.lock");
    file::delete("./.envrc");
}

if !variable::is_set("docker") {
    let docker = variable::prompt("Do you want to generate a Dockerfile for your gadget?", true);
    variable::set("docker", docker);
}

if check_bool("docker") {
    if !variable::is_set("base-image") {
        let base_image = variable::prompt("What base image should be used?", "rustlang/rust:nightly");
        variable::set("base-image", base_image);
    }
} else {
    file::delete("./Dockerfile");
}

if !variable::is_set("ci") {
    let ci = variable::prompt("Do you want to generate Github Actions workflows?", true);
    variable::set("ci", ci);
}

if check_bool("ci") {
    if !variable::is_set("rust-ci") {
        let rust_ci = variable::prompt("Create generic Rust workflows? (Rustfmt, Clippy, Cargo test)", true);
        variable::set("rust-ci", rust_ci);
    }

    if !variable::is_set("release-ci") {
        let release_ci = variable::prompt("Create a workflow for automatically releasing to Github releases?", true);
        variable::set("release-ci", release_ci);
    }
}

if !check_bool("rust-ci") {
    file::delete("./.github/workflows/ci.yml");
}

if !check_bool("release-ci") {
    file::delete("./.github/workflows/release.yml");
}

file::delete("./hooks");