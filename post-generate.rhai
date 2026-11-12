// The project CI workflow can't run in the template repo (Cargo.toml there has
// placeholders), so it is kept outside .github/workflows and moved into place
// in each generated project.
file::rename(".github/project-workflows/general.yml", ".github/workflows/general.yml");
file::delete(".github/project-workflows");
